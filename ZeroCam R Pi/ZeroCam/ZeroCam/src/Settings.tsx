import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {useEffect, useState} from "react";
import {Link} from "react-router-dom";
import "./global.css";
import {load, dump} from "js-yaml";
import {Config} from "./ConfigModel";


function Settings(){
  let [knownNetworks, setKnownNetworks] = useState<Array<string>>([])
  let [config, setConfig]               = useState<Config>()

  useEffect(() => {
    async function initKnownNetworks() {
      setKnownNetworks(await invoke('feGetKnownNetworks'))
    }
    initKnownNetworks()
  }, []);

  useEffect(() => {
    async function initConfig(){
      const fileContents: string = await invoke("feGetConfig");
      setConfig(load(fileContents) as Config);
    }
    initConfig()
  }, []);

  async function toggleFullscreen() {
    const fullScreenState = await getCurrentWindow().isFullscreen()
    await getCurrentWindow().setFullscreen(!fullScreenState)
  }

  async function saveAndReboot() {
    if(config){
      config.hotspot_networks = knownNetworks.filter(n =>
        (document.getElementById(`hotspot-checkbox-${n}`) as HTMLInputElement).checked
      )

      config.g_cloud.limit_gb = Number(extractField("g_cloud.limit_gb"));

      config.camera_input.clip.disk_full_buffer_gb = Number(extractField("camera_input.clip.disk_full_buffer_gb"));

      config.motion_listener.sensitivity_inverse  = Number(extractField("motion_listener.sensitivity_inverse"));
      config.motion_listener.threshold_sum_kilo   = Number(extractField("motion_listener.threshold_sum_kilo"));
      config.motion_listener.frame_delay_millisec = Number(extractField("motion_listener.frame_delay_millisec"));
      config.motion_listener.trigger_duration     = Number(extractField("motion_listener.trigger_duration"));

      config.camera_input.clip.segment_size_sec      = Number(extractField("camera_input.clip.segment_size_sec"));
      config.camera_input.clip.segments              = Number(extractField("camera_input.clip.segments"));
      config.camera_input.clip.timer_before_clip_sec = Number(extractField("camera_input.clip.timer_before_clip_sec"));
      config.camera_input.clip.cooldown_sec          = Number(extractField("camera_input.clip.cooldown_sec"));

      config.telegram_key = extractField("telegram_key").toString();
      config.internet_stream_output.url = extractField("internet_stream_output.url").toString();
    }

    const yamlContents = dump(config);
    await invoke("feSetConfig", {config: yamlContents});

    await invoke("feRebootSystem");
  }

  function extractField(fieldName: string): String{
    let element = (document.getElementById(fieldName) as HTMLInputElement)
    if(element.value == "")
      return element.placeholder
    else return element.value
  }

  function hotspotSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <thead><tr>
            <th className="h4" scope="col">Network:</th>
            <th className="h4" scope="col">Is Hotspot:</th>
        </tr></thead>
        <tbody>
          {knownNetworks.map(n => (
            <tr>
              <td className="h3 " id={n}>{n}</td>
              <td>
                <div className="form-check">
                  {config?.hotspot_networks?.includes(n) ?(
                    <input className="form-check-input" style={{width:"2em", height:"2em"}} type="checkbox" value="checked" id={`hotspot-checkbox-${n}`} defaultChecked={true}></input>
                  ) : (
                    <input className="form-check-input" style={{width:"2em", height:"2em"}} type="checkbox" value="" id={`hotspot-checkbox-${n}`} defaultChecked={false}></input>
                  )}
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    )
  }

  function gdriveSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <tbody>
          <tr>
            <td className="col h3">
              Storage Limit </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="g_cloud.limit_gb"
                placeholder={`${config?.g_cloud.limit_gb ?? ""}`}
                defaultValue={`${config?.g_cloud.limit_gb ?? ""}`}
              />
            </td>
            <td className="h3 w-25">GB</td>
          </tr>
        </tbody>
      </table>
    )
  }

  function localDriveSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <tbody>
          <tr>
            <td className="col h3">
              Storage Full Buffer </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="camera_input.clip.disk_full_buffer_gb"
                placeholder={`${config?.camera_input.clip.disk_full_buffer_gb ?? ""}`}
                defaultValue={`${config?.camera_input.clip.disk_full_buffer_gb ?? ""}`}
              />
            </td>
            <td className="h3 w-25">GB</td>
          </tr>
        </tbody>
      </table>
    )
  }

  function motionSensorSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <tbody>
          <tr>
            <td className="col h3">
              Sensitivity Inverse </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="motion_listener.sensitivity_inverse"
                placeholder={`${config?.motion_listener.sensitivity_inverse ?? ""}`}
                defaultValue={`${config?.motion_listener.sensitivity_inverse ?? ""}`}
              />
            </td>
            <td className="h3 w-25">0-255</td>
          </tr>
          <tr>
            <td className="col h3">
              Frame Difference Total </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="motion_listener.threshold_sum_kilo"
                placeholder={`${config?.motion_listener.threshold_sum_kilo ?? ""}`}
                defaultValue={`${config?.motion_listener.threshold_sum_kilo ?? ""}`}
              />
            </td>
            <td className="h3 w-25">0-4000</td>
          </tr>
          <tr>
            <td className="col h3">
              Delay between evaluating frames </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="motion_listener.frame_delay_millisec"
                placeholder={`${config?.motion_listener.frame_delay_millisec ?? ""}`}
                defaultValue={`${config?.motion_listener.frame_delay_millisec ?? ""}`}
              />
            </td>
            <td className="h3 w-25">milsec</td>
          </tr>
          <tr>
            <td className="col h3">
              Trigger Duration </td>
            <td className="w-25 pt-1 pb-1">
              <input className="form-control form-control-lg text-white" id="motion_listener.trigger_duration"
                placeholder={`${config?.motion_listener.trigger_duration ?? ""}`}
                defaultValue={`${config?.motion_listener.trigger_duration ?? ""}`}
              />
            </td>
            <td className="h3 w-25">ticks</td>
          </tr>
        </tbody>
      </table>
    )
  }

  function videoSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <tbody>
        <tr>
          <td className="col h3">
            Segment Length </td>
          <td className="w-25 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="camera_input.clip.segment_size_sec"
              placeholder={`${config?.camera_input.clip.segment_size_sec ?? ""}`}
              defaultValue={`${config?.camera_input.clip.segment_size_sec ?? ""}`}
            />
          </td>
          <td className="h3 w-25">sec</td>
        </tr>
        <tr>
          <td className="col h3">
            Segments </td>
          <td className="w-25 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="camera_input.clip.segments"
              placeholder={`${config?.camera_input.clip.segments ?? ""}`}
              defaultValue={`${config?.camera_input.clip.segments ?? ""}`}
            />
          </td>
          <td className="h3 w-25"></td>
        </tr>
        <tr>
          <td className="col h3">
            Timer before clip </td>
          <td className="w-25 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="camera_input.clip.timer_before_clip_sec"
              placeholder={`${config?.camera_input.clip.timer_before_clip_sec ?? ""}`}
              defaultValue={`${config?.camera_input.clip.timer_before_clip_sec ?? ""}`}
            />
          </td>
          <td className="h3 w-25">sec</td>
        </tr>
        <tr>
          <td className="col h3">
            Clip cooldown </td>
          <td className="w-25 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="camera_input.clip.cooldown_sec"
              placeholder={`${config?.camera_input.clip.cooldown_sec ?? ""}`}
              defaultValue={`${config?.camera_input.clip.cooldown_sec ?? ""}`}
            />
          </td>
          <td className="h3 w-25">sec</td>
        </tr>
        </tbody>
      </table>
    )
  }

  function telegramSettings() {
    return(
      <table id="known-networks-table" className="mt-0 pt-0 table table-striped">
        <tbody>
        <tr>
          <td className="col h3">
            API Key </td>
          <td className="w-50 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="telegram_key"
              placeholder={`${config?.telegram_key ?? ""}`}
              defaultValue={`${config?.telegram_key ?? ""}`}
            />
          </td>
        </tr>
        <tr>
          <td className="col h3">
            Stream output URL </td>
          <td className="w-50 pt-1 pb-1">
            <input className="form-control form-control-lg text-white" id="internet_stream_output.url"
              placeholder={`${config?.internet_stream_output.url ?? ""}`}
              defaultValue={`${config?.internet_stream_output.url ?? ""}`}
            />
          </td>
        </tr>
        </tbody>
      </table>
    )
  }

  return (
    <body>
      <nav className="row ps-3 pe-3 navbar bg-dark fixed-top">
        <div className="col">
          <Link to = {"/"}>
            <button type="button" id="return-button" className="btn btn-outline-light h-100 w-100">
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                   className="bi bi-arrow-bar-left" viewBox="0 0 16 16" style={{width: "4em", height: "4em"}}>
                <path fillRule="evenodd"
                      d="M12.5 15a.5.5 0 0 1-.5-.5v-13a.5.5 0 0 1 1 0v13a.5.5 0 0 1-.5.5M10 8a.5.5 0 0 1-.5.5H3.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L3.707 7.5H9.5a.5.5 0 0 1 .5.5"/>
              </svg>
            </button>
          </Link>
        </div>
        <div className="col">
          <button type="button" id="save-and-reboot-button" className="btn pt-0 pb-0 btn-danger btn-outline-light h-100 w-100" onClick={saveAndReboot}>
            <div className="row">
              <div className="col ps-0 pe-0 pt-1 pb-1 d-flex align-items-center justify-content-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                     className="bi bi-sd-card" viewBox="0 0 16 16" style={{width: "4em", height: "4em"}}>
                  <path
                    d="M6.25 3.5a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0z"/>
                  <path fillRule="evenodd"
                        d="M5.914 0H12.5A1.5 1.5 0 0 1 14 1.5v13a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 14.5V3.914c0-.398.158-.78.44-1.06L4.853.439A1.5 1.5 0 0 1 5.914 0M13 1.5a.5.5 0 0 0-.5-.5H5.914a.5.5 0 0 0-.353.146L3.146 3.561A.5.5 0 0 0 3 3.914V14.5a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5z"/>
                </svg>
              </div>
              <div className="col ps-0 pe-0 mt-0 mb-0 pt-0 pb-0">
                <span className="h2 weight-bold ">Save & &nbsp;</span>
                <span className="h2 weight-bold">Reboot</span>
              </div>
              <div className="col ps-0 pe-0 pt-1 pb-1 d-flex align-items-center justify-content-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                     className="bi bi-arrow-clockwise" viewBox="0 0 16 16" style={{width: "4em", height: "4em"}}>
                  <path fillRule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z"/>
                  <path
                    d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466"/>
                </svg>
              </div>
            </div>
          </button>
        </div>
        <div className="col">
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
      </nav>
      <main role="main" className="container-fluid" style={{paddingTop:"80px"}}>
        <div className="row ms-3 me-3">
          <h1 className="ps-0 mt-3 display-6">Known Networks:</h1>
          {hotspotSettings()}
          <h1 className="ps-0 mt-3 display-6">Google Drive:</h1>
          {gdriveSettings()}
          <h1 className="ps-0 mt-3 display-6">Local Drive:</h1>
          {localDriveSettings()}
          <h1 className="ps-0 mt-3 display-6">Motion Detection:</h1>
          {motionSensorSettings()}
          <h1 className="ps-0 mt-3 display-6">Video Capture:</h1>
          {videoSettings()}
          <h1 className="ps-0 mt-3 display-6">Telegram:</h1>
          {telegramSettings()}
        </div>
      </main>
    </body>
  )
}

export default Settings;