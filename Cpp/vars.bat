set CURRENT_INSTANCE_DIR=%~dp0
set CURRENT_INSTANCE_DIR=%CURRENT_INSTANCE_DIR:\=/%
IF %CURRENT_INSTANCE_DIR:~-1%==/ SET CURRENT_INSTANCE_DIR=%CURRENT_INSTANCE_DIR:~0,-1%
echo Current instance dir: %CURRENT_INSTANCE_DIR%

set BUILD_DIR=%CURRENT_INSTANCE_DIR%/build
echo Current build dir: %BUILD_DIR%

set PATH=C:/Program Files/LLVM/bin;%PATH%
