#!/bin/bash
set -e

# Install jogger-macos as a LaunchAgent to start at login

PLIST_NAME="com.jogger.macos.plist"
PLIST_PATH="$HOME/Library/LaunchAgents/$PLIST_NAME"
BINARY_PATH="$HOME/.cargo/bin/jogger-macos"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ jogger-macos not found at $BINARY_PATH"
    echo "Run: cargo install --path jogger-macos"
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
        <string>$BINARY_PATH</string>
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
echo "🏃 Binary: $BINARY_PATH"
echo "📝 Logs: $HOME/Library/Logs/jogger-macos.log"
echo ""
echo "To uninstall:"
echo "  launchctl unload $PLIST_PATH"
echo "  rm $PLIST_PATH"
