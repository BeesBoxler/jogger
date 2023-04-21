#!/bin/sh

echo Creating pre-commit hook...

cat > .git/hooks/pre-commit <<\EOF
#!/bin/sh

cargo fmt &&\
cargo clippy &&\
cargo test;
EOF

chmod +x .git/hooks/pre-commit

echo Done.