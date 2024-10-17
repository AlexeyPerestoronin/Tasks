call %~dp0vars.bat

cd %BUILD_DIR%
ninja -j4 all