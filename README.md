# ZeroCam Developer Guide for Ubuntu v24 LTS and Raspberry Pi 4B

## Dependencies Installation
#### Skip this if you are using the RaspberryPi image
- ### Install Rust
  - https://www.rust-lang.org/tools/install
    - ```bash 
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
- ### Install Tauri
  - PreRequisites
    - https://tauri.app/start/prerequisites/#linux
    - ```cargo tauri add cli```
- ### Instal npm
- ### Install libfuse2
- ### Install ffmpeg dependencies
  ```bash 
    sudo apt-get install build-essential clang libclang-dev
  ```
- ### Install ffmpeg
  ```bash 
    sudo apt install ffmpeg
    ./configure --enable-shared --enable-libx264 --enable-gpl
    make
    make install
  ```
- ### Install v4l2 loopback
  - https://docs.omnissa.com/bundle/LinuxDesktops-and-Applications-in-HorizonV2306/page/InstalltheV4L2LoopbackDriver.html
  - On raspberry pi this dependency will break upon building, you will need open `v4l2loopback.c` file 
    after cloning from github and add `#include <linux/string.h>` at the top and convert all usage 
    of `strlcpy` to `strncpy` save and then build
- ### Install opencv4
  - https://docs.opencv.org/4.x/d7/d9f/tutorial_linux_install.html
- ### Install other open cv dependencies
  - https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md
  - ### Install libsdl2-2.0-0
    - ```bash
      sudo apt-get install libsdl2-2.0-0
    ```
    
# Raspberry Pi Image installation
#### Skip this step if you did manual dependency installation
Follow this guide https://www.raspberrypi.com/documentation/computers/getting-started.html.
Instead of selecting an official Raspberry Pi OS, use ZeroCam.img provided.

- # Setup
- ### Google Drive
  - Create new google account
  - Go to Google Cloud Console
      - https://console.cloud.google.com/welcome/new
  - Search "Google Drive" and enable API
  - Create Oauth 2.0 Client
  - Select desktop application
      - ![Screenshot from 2025-02-14 14-27-10.png](DocsResources/Screenshot%20from%202025-02-14%2014-27-10.png)
  - Download the file
      - ![Screenshot from 2025-02-11 17-19-05.png](DocsResources/Screenshot%20from%202025-02-11%2017-19-05.png)
  - Move it into your ZeroCam/Zerocam/lib or zerocam_0.0.0_amd64/data/usr/lib/zerocam built folder
      - ![Screenshot from 2025-02-11 17-22-13.png](DocsResources/Screenshot%20from%202025-02-11%2017-22-13.png)
  - Rename the file to secret.json
      - ![Pasted image (7).png](DocsResources/Pasted%20image%20%287%29.png)
  - 
- ### Disable password for sudo actions (developer setup)
  - https://askubuntu.com/questions/147241/execute-sudo-without-password
- ### (Option 1) Install playit.gg to prevent need for port forwarding for each wifi source
  - Create https://playit.gg/ account and verify email (guest will not work)
  - ```bash
    wget https://github.com/playit-cloud/playit-agent/releases/download/v0.15.0/playit-linux-amd64
    chmod +x playit-linux-amd64
    ./playit-linux-amd64
    ```
  - Click the link shown in terminal
  - Click create tunnel and copy these settings
    - ![Pasted image (2).png](DocsResources/Pasted%20image%20%282%29.png)
  - Save the address in the config in output URL file
    - ![Pasted image (8).png](DocsResources/Pasted%20image%20%288%29.png)
- ### (Option 2 if you have an existing domain)
  - Install cloudflared https://pkg.cloudflare.com/index.html
  - https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/get-started/create-remote-tunnel/
  - Set it up tunnel as shown and follow guide
    - https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/get-started/create-remote-tunnel/
      - ![Pasted image (3).png](DocsResources/Pasted%20image%20%283%29.png)
  - ```bash 
    cloudflared tunnel login
    ```
- ### Setup Telegram Bot
  - On Telegram search "BotFather"
    - ![Pasted image.png](../DocsResources/Pasted%20image.png)
  - Start
  - ```/newbot```
  - ```ZeroCam```
  - give the bot a unique name
  - Copy the HTTP API token eg. 7805646492:AAEfzYJXfaeS9giXfPC1Dwy9efVBHFrGIdA
  - Save the new bot in your telegram eg. t.me/ZeroCam02032025bot
  - Save the key to inside settings on ZeroCam
    - ![Pasted image (8).png](DocsResources/Pasted%20image%20%288%29.png)
    
## Run

- #### Raspberry Pi 4B
  - Raspberry pi opencv api is different for arm64 architecture, if you want to build a new version of the project
    you will need to move it onto a raspberry pi and build it there. You will also need to modify add these lines for a 
    successful arm64 build.
    - ![Pasted image (10).png](DocsResources/Pasted%20image%20%2810%29.png)
    - ![Pasted image (11).png](DocsResources/Pasted%20image%20%2811%29.png)
    - ![Pasted image (12).png](DocsResources/Pasted%20image%20%2812%29.png)
  - Setup launch on boot
    - https://www.dexterindustries.com/howto/run-a-program-on-your-raspberry-pi-at-startup/

- #### (First time) In terminal from /ZeroCam
  - ```bash 
      ./TokenGeneratorScript
    ```
    - There will be a message in terminal asking you to go to browser,
      click the link and login with your dedicated dashcam google cloud account.
      - ![Pasted image (4).png](DocsResources/Pasted%20image%20%284%29.png)
  
- #### From inside /ZeroCam/ZeroCam
  - run 
    - ```bash
      npm run tauri dev
      ```
  - run with logs 
    - ```bash
      RUST_LOG=INFO npm run tauri dev
      ```
  - run with debug logs
    - ```bash
      RUST_LOG=DEBUG npm run tauri dev
      ```
      
- #### Built
  - got to zerocam_0.0.0_amd64/data/usr/lib/zerocam folder
    - ![Pasted image (9).png](DocsResources/Pasted%20image%20%289%29.png)
  - run from terminal