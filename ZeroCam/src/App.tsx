
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import "./global.css";
import Hls from "hls.js"
import {useEffect, useRef, useState} from "react";
import {Link} from "react-router-dom";
import icon64 from './assets/64x64.png';

function App() {
  const videoRef = useRef<HTMLVideoElement>(null);
  let [parked, setParked] = useState<boolean>(false)

  function scheduleClip() {
    invoke('feScheduleClip');
  }

  function makeParked(p: boolean) {
    invoke('feSetParked', {parked: p});
    setParked(p);
  }

  async function toggleFullscreen() {
    const fullScreenState = await getCurrentWindow().isFullscreen()
    await getCurrentWindow().setFullscreen(!fullScreenState)
  }

  useEffect(() => {
    async function getParked() {
      try {
        setParked(await invoke('feGetParked'));
      } catch (e){
        console.error(e);
      }
    }
    getParked();
    const intervalId = setInterval(getParked, 2000);
    return () => clearInterval(intervalId);
  }, []);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    const hls = new Hls({
      lowLatencyMode : true
    });

    const loadStream = () => {
      hls.loadSource("http://localhost:8888/stream1/index.m3u8");
      hls.attachMedia(video);
    };

    hls.on(Hls.Events.ERROR, (_, data) => {
      if (data.fatal) {
        console.warn("Stream error, retrying in 3 seconds...", data);
        setTimeout(loadStream, 3000);
      }
    });

    loadStream();
    video.controls = false;

    return () => {
      hls.destroy();
    };
  }, []);

  return (
    <main className="container-fluid">
      <div className="row mb-2 pt-2 ml-3 d-flex align-items-center justify-content-between">
        <div className="col">
          <Link to = {"/settings"}>
            <button type="button" id="settings-button" className="btn btn-outline-light h-100 w-100">
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" className="bi bi-gear" viewBox="0 0 16 16"
                   style={{ width: "4em", height: "4em" }}>
                <path
                  d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492M5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0"/>
                <path
                  d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52zm-2.633.283c.246-.835 1.428-.835 1.674 0l.094.319a1.873 1.873 0 0 0 2.693 1.115l.291-.16c.764-.415 1.6.42 1.184 1.185l-.159.292a1.873 1.873 0 0 0 1.116 2.692l.318.094c.835.246.835 1.428 0 1.674l-.319.094a1.873 1.873 0 0 0-1.115 2.693l.16.291c.415.764-.42 1.6-1.185 1.184l-.291-.159a1.873 1.873 0 0 0-2.693 1.116l-.094.318c-.246.835-1.428.835-1.674 0l-.094-.319a1.873 1.873 0 0 0-2.692-1.115l-.292.16c-.764.415-1.6-.42-1.184-1.185l.159-.291A1.873 1.873 0 0 0 1.945 8.93l-.319-.094c-.835-.246-.835-1.428 0-1.674l.319-.094A1.873 1.873 0 0 0 3.06 4.377l-.16-.292c-.415-.764.42-1.6 1.185-1.184l.292.159a1.873 1.873 0 0 0 2.692-1.115z"/>
              </svg>
            </button>
          </Link>
        </div>

        <div className="col p-0 m-0 d-flex align-items-center justify-content-center">
          <img className="img-fluid d-block" src={icon64} style={{ width: "4em", height: "4em"}}/>
          &nbsp;&nbsp;
          <span className="display-6">ZeroCam</span>
        </div>


        <div className="col d-flex justify-content-center align-items-center">
          <button type="button" id="window-button" className="btn btn-outline-light h-100 w-100" onClick={toggleFullscreen}>
            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" className="bi bi-aspect-ratio"
                 viewBox="0 0 16 16" style={{ width: "4em", height: "4em" }}>
              <path
                d="M0 3.5A1.5 1.5 0 0 1 1.5 2h13A1.5 1.5 0 0 1 16 3.5v9a1.5 1.5 0 0 1-1.5 1.5h-13A1.5 1.5 0 0 1 0 12.5zM1.5 3a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5h13a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5z"/>
              <path
                d="M2 4.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 0 1H3v2.5a.5.5 0 0 1-1 0zm12 7a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1 0-1H13V8.5a.5.5 0 0 1 1 0z"/>
            </svg>
          </button>
        </div>

      </div>
      <div className="row my-auto ms-0">
        <div className="col-4 ps-0 pe-0 flex-column d-grid">
          <button type="button" className="btn btn-outline-light me-0 d-flex align-items-center justify-content-center" id="clip-button" onClick={scheduleClip}>
            <span className="display-2">CLIP </span>
            <div>&nbsp;&nbsp;&nbsp;&nbsp;</div>
            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" className="bi bi-camera-video"
                 viewBox="0 0 16 16" style={{width:"4.5em", height:"4.5em"}}>
              <path fillRule="evenodd" d="M0 5a2 2 0 0 1 2-2h7.5a2 2 0 0 1 1.983 1.738l3.11-1.382A1 1 0 0 1 16 4.269v7.462a1 1 0 0 1-1.406.913l-3.111-1.382A2 2 0 0 1 9.5 13H2a2 2 0 0 1-2-2zm11.5 5.175 3.5 1.556V4.269l-3.5 1.556zM2 4a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h7.5a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1z"/>
            </svg>
          </button>
        </div>

        <div className="col-8">
          <div className="ratio ratio-16x9">
            <video ref={videoRef} id="video" className="object-fit-contain" autoPlay muted></video>
          </div>
        </div>
      </div>



      <div className="row m-0 mt-2 mb-0">
        {parked? (
          <button type="button" id="park-button"
                  className="pt-2 pb-2 btn btn-primary btn-outline-light d-flex align-items-center justify-content-center" onClick={() => makeParked(false)}>
            <span className="display-3 weight-bold">PARKED</span>
            <div>&nbsp;&nbsp;&nbsp;&nbsp;</div>
            <svg className="bi bi-car-front-fill" xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                 viewBox="0 0 16 16" style={{width:"3em", height:"3em"}}>
              <path
                d="M2.52 3.515A2.5 2.5 0 0 1 4.82 2h6.362c1 0 1.904.596 2.298 1.515l.792 1.848c.075.175.21.319.38.404.5.25.855.715.965 1.262l.335 1.679q.05.242.049.49v.413c0 .814-.39 1.543-1 1.997V13.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-1.338c-1.292.048-2.745.088-4 .088s-2.708-.04-4-.088V13.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-1.892c-.61-.454-1-1.183-1-1.997v-.413a2.5 2.5 0 0 1 .049-.49l.335-1.68c.11-.546.465-1.012.964-1.261a.8.8 0 0 0 .381-.404l.792-1.848ZM3 10a1 1 0 1 0 0-2 1 1 0 0 0 0 2m10 0a1 1 0 1 0 0-2 1 1 0 0 0 0 2M6 8a1 1 0 0 0 0 2h4a1 1 0 1 0 0-2zM2.906 5.189a.51.51 0 0 0 .497.731c.91-.073 3.35-.17 4.597-.17s3.688.097 4.597.17a.51.51 0 0 0 .497-.731l-.956-1.913A.5.5 0 0 0 11.691 3H4.309a.5.5 0 0 0-.447.276L2.906 5.19Z"/>
            </svg>
          </button>
        ) : (
          <button type="button" id="park-button"
                  className="pt-2 pb-2 btn btn-secondary btn-outline-light d-flex align-items-center justify-content-center" onClick={() => makeParked(true)}>
            <span className="display-3 weight-bold">DRIVING</span>
            <div>&nbsp;&nbsp;&nbsp;&nbsp;</div>
            <svg className="bi bi-car-front-fill" xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                 viewBox="0 0 16 16" style={{width:"3em", height:"3em"}}>
              <path
                d="M2.52 3.515A2.5 2.5 0 0 1 4.82 2h6.362c1 0 1.904.596 2.298 1.515l.792 1.848c.075.175.21.319.38.404.5.25.855.715.965 1.262l.335 1.679q.05.242.049.49v.413c0 .814-.39 1.543-1 1.997V13.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-1.338c-1.292.048-2.745.088-4 .088s-2.708-.04-4-.088V13.5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1-.5-.5v-1.892c-.61-.454-1-1.183-1-1.997v-.413a2.5 2.5 0 0 1 .049-.49l.335-1.68c.11-.546.465-1.012.964-1.261a.8.8 0 0 0 .381-.404l.792-1.848ZM3 10a1 1 0 1 0 0-2 1 1 0 0 0 0 2m10 0a1 1 0 1 0 0-2 1 1 0 0 0 0 2M6 8a1 1 0 0 0 0 2h4a1 1 0 1 0 0-2zM2.906 5.189a.51.51 0 0 0 .497.731c.91-.073 3.35-.17 4.597-.17s3.688.097 4.597.17a.51.51 0 0 0 .497-.731l-.956-1.913A.5.5 0 0 0 11.691 3H4.309a.5.5 0 0 0-.447.276L2.906 5.19Z"/>
            </svg>
          </button>
        )}
      </div>
    </main>
  )
}

export default App;
