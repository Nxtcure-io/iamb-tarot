@echo off
REM AstroTTY Setup Script for Windows
REM This script sets up the AstroTTY tarot terminal client

setlocal enabledelayedexpansion

echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║                                                           ║
echo ║              🔮 AstroTTY Setup Wizard 🔮                 ║
echo ║                                                           ║
echo ║         Terminal-based Tarot Reading Client              ║
echo ║                                                           ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.
echo.

REM Check for admin rights
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [WARNING] This script may require administrator privileges for some operations.
    echo.
)

REM Check if Rust is installed
echo [1/7] Checking Rust installation...
rustc --version >nul 2>&1
if %errorLevel% equ 0 (
    for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
    echo [OK] Rust is already installed: !RUST_VERSION!
) else (
    echo Rust not found. Installing Rust...
    echo.
    echo Please follow the instructions in the browser window that will open.
    echo After installation, restart this script.
    start https://rustup.rs/
    pause
    exit /b 1
)
echo.

REM Check for Visual Studio Build Tools
echo [2/7] Checking build tools...
where cl.exe >nul 2>&1
if %errorLevel% neq 0 (
    echo [WARNING] Visual Studio Build Tools not found.
    echo You may need to install Visual Studio Build Tools for C++
    echo Download from: https://visualstudio.microsoft.com/downloads/
    echo.
    echo Do you want to continue anyway? (Y/N)
    set /p CONTINUE=
    if /i not "!CONTINUE!"=="Y" exit /b 1
)
echo [OK] Build tools available
echo.

REM Get user credentials
echo [3/7] Setting up Matrix account...
echo.
echo Please enter your desired username (lowercase, no spaces):
set /p USERNAME=
echo.

REM Validate username
echo !USERNAME! | findstr /r "^[a-z0-9_-]*$" >nul
if %errorLevel% neq 0 (
    echo [ERROR] Username must contain only lowercase letters, numbers, hyphens, and underscores
    pause
    exit /b 1
)

set MATRIX_ID=@!USERNAME!:endlessperfect.com

REM Check if username exists
echo Checking username availability...
curl -s -o nul -w "%%{http_code}" "https://endlessperfect.com/tarot-api/api/users/!MATRIX_ID!" > temp_http_code.txt
set /p HTTP_CODE=<temp_http_code.txt
del temp_http_code.txt

if "!HTTP_CODE!"=="200" (
    echo [WARNING] Username '!USERNAME!' already exists in the tarot database.
    echo Do you want to use this existing account? (Y/N)
    set /p USE_EXISTING=
    if /i not "!USE_EXISTING!"=="Y" (
        echo Please run the setup again with a different username.
        pause
        exit /b 1
    )
) else (
    echo [OK] Username '!USERNAME!' is available
)

echo.
echo Please enter your password:
set "PASSWORD="
call :GetPassword PASSWORD
echo.
echo Confirm password:
set "PASSWORD_CONFIRM="
call :GetPassword PASSWORD_CONFIRM
echo.

if not "!PASSWORD!"=="!PASSWORD_CONFIRM!" (
    echo [ERROR] Passwords do not match
    pause
    exit /b 1
)

echo [OK] Credentials configured
echo.

REM Build the application
echo [4/7] Building AstroTTY (this may take several minutes)...
cargo build --release
if %errorLevel% neq 0 (
    echo [ERROR] Build failed
    pause
    exit /b 1
)
echo [OK] Build complete
echo.

REM Create config directory
echo [5/7] Creating configuration...
set CONFIG_DIR=%USERPROFILE%\.config\iamb
if not exist "%CONFIG_DIR%" mkdir "%CONFIG_DIR%"

REM Detect terminal for image protocol
set IMAGE_PROTOCOL=halfblocks
if defined WT_SESSION (
    set IMAGE_PROTOCOL=sixel
    echo [OK] Detected Windows Terminal - using sixel protocol
) else (
    echo Using halfblocks for image display (works in any terminal^)
    echo For better image quality, consider using Windows Terminal or Kitty
)

REM Create config file
(
echo # AstroTTY Configuration
echo default_profile = "!USERNAME!"
echo.
echo [profiles.!USERNAME!]
echo user_id = "!MATRIX_ID!"
echo url = "https://endlessperfect.com"
echo.
echo # General settings
echo [settings]
echo reaction_display = true
echo read_receipt_send = true
echo typing_notice_send = true
echo.
echo # Image preview settings
echo [settings.image_preview]
) > "%CONFIG_DIR%\config.toml"

if "!IMAGE_PROTOCOL!"=="sixel" (
    (
    echo protocol.type = "sixel"
    echo size = { width = 80, height = 20 }
    ) >> "%CONFIG_DIR%\config.toml"
) else (
    (
    echo protocol.type = "halfblocks"
    echo size = { width = 80, height = 20 }
    ) >> "%CONFIG_DIR%\config.toml"
)

echo [OK] Configuration created at %CONFIG_DIR%\config.toml
echo.

REM Install binary
echo [6/7] Installing AstroTTY...
set INSTALL_DIR=%USERPROFILE%\.local\bin
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
copy /Y target\release\iamb.exe "%INSTALL_DIR%\astrotty.exe" >nul
echo [OK] AstroTTY installed to %INSTALL_DIR%\astrotty.exe

REM Add to PATH
echo %PATH% | findstr /C:"%INSTALL_DIR%" >nul
if %errorLevel% neq 0 (
    echo.
    echo [INFO] Adding %INSTALL_DIR% to user PATH...
    setx PATH "%PATH%;%INSTALL_DIR%"
    echo [OK] Added to PATH
    echo [NOTE] You may need to restart your terminal for the PATH change to take effect
)
echo.

REM Register with Matrix server
echo [7/7] Registering with Matrix server...

REM Create JSON for registration
echo {"username":"!USERNAME!","password":"!PASSWORD!","auth":{"type":"m.login.dummy"}} > temp_register.json

curl -s -X POST "https://endlessperfect.com/_matrix/client/r0/register" ^
    -H "Content-Type: application/json" ^
    -d @temp_register.json > temp_response.json

findstr /C:"user_id" temp_response.json >nul
if %errorLevel% equ 0 (
    echo [OK] Matrix account created
) else (
    findstr /C:"User ID already taken" temp_response.json >nul
    if %errorLevel% equ 0 (
        echo [INFO] Matrix account already exists, will use existing account
    ) else (
        echo [INFO] Matrix registration may require admin approval
    )
)

REM Create tarot database user
if not "!HTTP_CODE!"=="200" (
    echo {"matrix_id":"!MATRIX_ID!","username":"!USERNAME!"} > temp_tarot_user.json
    curl -s -X POST "https://endlessperfect.com/tarot-api/api/users" ^
        -H "Content-Type: application/json" ^
        -d @temp_tarot_user.json > temp_tarot_response.json
    
    findstr /C:"user_id" temp_tarot_response.json >nul
    if %errorLevel% equ 0 (
        echo [OK] Tarot database user created
    )
    del temp_tarot_user.json temp_tarot_response.json
)

del temp_register.json temp_response.json

echo.
echo ╔═══════════════════════════════════════════════════════════╗
echo ║                                                           ║
echo ║              ✨ Setup Complete! ✨                       ║
echo ║                                                           ║
echo ╚═══════════════════════════════════════════════════════════╝
echo.
echo.
echo Getting Started:
echo.
echo 1. Start AstroTTY:
echo    astrotty
echo.
echo 2. Login with your credentials when prompted
echo.
echo 3. Try these commands in any room:
echo    :tarot 3              - Draw a 3-card spread
echo    :tarot fool           - Look up a specific card
echo    :tarothistory         - View your reading history
echo    :tarothistory suits   - See suit distribution
echo    :tarothistory summary - View analytics
echo.
echo Your Account:
echo    Username: !USERNAME!
echo    Matrix ID: !MATRIX_ID!
echo    Server: https://endlessperfect.com
echo.
echo Configuration:
echo    Config: %CONFIG_DIR%\config.toml
echo    Binary: %INSTALL_DIR%\astrotty.exe
echo    Image Protocol: !IMAGE_PROTOCOL!
echo.
echo [NOTE] If 'astrotty' command is not found, restart your terminal
echo.
echo For help and documentation, visit:
echo https://github.com/yourusername/iamb-tarot
echo.
echo Happy reading! 🔮✨
echo.
pause
exit /b 0

REM Function to read password without echoing
:GetPassword
setlocal enabledelayedexpansion
set "psCommand=powershell -Command "$pword = read-host -AsSecureString ; ^
    $BSTR=[System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($pword); ^
    [System.Runtime.InteropServices.Marshal]::PtrToStringAuto($BSTR)""
for /f "usebackq delims=" %%p in (`%psCommand%`) do set "password=%%p"
endlocal & set "%~1=%password%"
goto :eof
