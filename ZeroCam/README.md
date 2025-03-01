# ZeroCam

## Setup
- ### Install Rust
  - https://www.rust-lang.org/tools/install
    - ```bash 
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
- Create new google account
- Go to Google Cloud Console
  - https://console.cloud.google.com/welcome/new
- Search "Google Drive" and enable API
- Create Oauth 2.0 Client
- Select desktop application
  ![Screenshot from 2025-02-14 14-27-10.png](DocsResources/Screenshot%20from%202025-02-14%2014-27-10.png)
- Download the file
  ![Screenshot from 2025-02-11 17-19-05.png](DocsResources/Screenshot%20from%202025-02-11%2017-19-05.png)
- Move it into your ZeroCam folder
  ![Screenshot from 2025-02-11 17-22-13.png](DocsResources/Screenshot%20from%202025-02-11%2017-22-13.png)
- Rename the file to secret.json
  - ![Screenshot from 2025-02-14 14-30-03.png](DocsResources/Screenshot%20from%202025-02-14%2014-30-03.png)
- ### Install Tauri
  - PreRequisites
    - https://tauri.app/start/prerequisites/#linux
    - ```cargo tauri add cli```
- ### Instal npm
  - 
- ### Install libfuse2
  - 
- Install ffmpeg dependencies
  ```bash 
    sudo apt-get install build-essential clang libclang-dev
  ```
- ### Install ffmpeg
  ```bash 
    sudo apt install ffmpeg
  ```
- ### Install MediaMTX into zerocam folder (for streaming)
  - https://github.com/bluenviron/mediamtx/releases
  - rename folder to "MediaMTX"

## Run
- ```bash
    cargo run  
  ```
- (First time)
  - There will be a message asking you to go to browser,
    click the link and login with your dedicated dashcam google cloud account.
    ![Screenshot from 2025-02-14 14-35-17.png](DocsResources/Screenshot%20from%202025-02-14%2014-35-17.png)
  