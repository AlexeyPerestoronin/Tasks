call %~dp0vars.bat

call cmake ^
-GNinja ^
-DCMAKE_C_COMPILER=clang ^
-DCMAKE_CXX_COMPILER=clang++ ^
-DCMAKE_CXX_STANDARD=17 ^
-DCMAKE_BUILD_TYPE=Debug ^
-S %CURRENT_INSTANCE_DIR% ^
-B %BUILD_DIR%