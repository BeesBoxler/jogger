#!/bin/bash
set -euo pipefail

# Install jogger-macos as a LaunchAgent to start at login.
# Usage:
#   ./install-launchagent.sh
#   ./install-launchagent.sh /Applications/Jogger.app

PLIST_NAME="com.jogger.macos.plist"
PLIST_PATH="$HOME/Library/LaunchAgents/$PLIST_NAME"

if [ "${1:-}" != "" ]; then
    APP_PATH="$1"
elif [ -d "/Applications/Jogger.app" ]; then
    APP_PATH="/Applications/Jogger.app"
elif [ -d "$HOME/Applications/Jogger.app" ]; then
    APP_PATH="$HOME/Applications/Jogger.app"
else
    echo "❌ Jogger.app not found."
    echo ""
    echo "Expected one of:"
    echo "  - /Applications/Jogger.app"
    echo "  - $HOME/Applications/Jogger.app"
    echo ""
    echo "Install with Homebrew (recommended):"
    echo "  brew install --cask beesboxler/jogger/jogger-macos"
    echo ""
    echo "Or pass an explicit path:"
    echo "  ./install-launchagent.sh /path/to/Jogger.app"
    exit 1
fi

mkdir -p "$HOME/Library/LaunchAgents"

cat > "$PLIST_PATH" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.jogger.macos</string>
    
    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/open</string>
        <string>$APP_PATH</string>
    </array>
    
    <key>RunAtLoad</key>
    <true/>
    
    <key>KeepAlive</key>
    <false/>
    
    <key>StandardOutPath</key>
    <string>$HOME/Library/Logs/jogger-macos.log</string>
    
    <key>StandardErrorPath</key>
    <string>$HOME/Library/Logs/jogger-macos.error.log</string>
</dict>
</plist>
EOF

# Load/reload the LaunchAgent (modern launchctl first, with legacy fallback).
launchctl bootout "gui/$(id -u)" "$PLIST_PATH" 2>/dev/null || true
launchctl bootstrap "gui/$(id -u)" "$PLIST_PATH" 2>/dev/null || {
    launchctl unload "$PLIST_PATH" 2>/dev/null || true
    launchctl load "$PLIST_PATH"
}

echo "✅ Jogger installed as LaunchAgent"
echo "📍 Plist: $PLIST_PATH"
echo "🏃 App: $APP_PATH"
echo "📝 Logs: $HOME/Library/Logs/jogger-macos.log"
echo ""
echo "To uninstall:"
echo "  launchctl unload $PLIST_PATH"
echo "  rm $PLIST_PATH"
