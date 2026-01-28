#!/bin/bash
set -e

# Install jogger-macos as a LaunchAgent to start at login

PLIST_NAME="com.jogger.macos.plist"
PLIST_PATH="$HOME/Library/LaunchAgents/$PLIST_NAME"
APP_PATH="$HOME/Applications/Jogger.app"

# Check if app bundle exists
if [ ! -d "$APP_PATH" ]; then
    echo "❌ Jogger.app not found at $APP_PATH"
    echo ""
    echo "Please install Jogger.app first:"
    echo "  1. Download from GitHub releases"
    echo "  2. Extract: tar xzf jogger-macos-*.tar.gz"
    echo "  3. Move to Applications: mv Jogger.app ~/Applications/"
    echo ""
    echo "Or build locally:"
    echo "  ./build-app-bundle.sh"
    echo "  mv target/release/Jogger.app ~/Applications/"
    exit 1
fi

# Create LaunchAgents directory if it doesn't exist
mkdir -p "$HOME/Library/LaunchAgents"

# Create the plist file
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

# Load the LaunchAgent
launchctl unload "$PLIST_PATH" 2>/dev/null || true
launchctl load "$PLIST_PATH"

echo "✅ Jogger installed as LaunchAgent"
echo "📍 Plist: $PLIST_PATH"
echo "🏃 App: $APP_PATH"
echo "📝 Logs: $HOME/Library/Logs/jogger-macos.log"
echo ""
echo "To uninstall:"
echo "  launchctl unload $PLIST_PATH"
echo "  rm $PLIST_PATH"
