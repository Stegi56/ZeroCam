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
    }

    const yamlContents = dump(config)
    await invoke("feSetConfig", {config: yamlContents})

    //invoke("feRebootSystem")
  }

  function hotspotSelector() {
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
                  {config?.hotspot_networks.includes(n) ?(
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

  return(
    <main className="container-fluid pt-2">
      <div className="row">
        <div className="col">
          <Link to = {"/"}>
            <button type="button" id="return-button" className="btn btn-outline-light h-100 w-100">
              <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                   className="bi bi-arrow-bar-left" viewBox="0 0 16 16" style={{width: "3em", height: "3em"}}>
                <path fillRule="evenodd"
                      d="M12.5 15a.5.5 0 0 1-.5-.5v-13a.5.5 0 0 1 1 0v13a.5.5 0 0 1-.5.5M10 8a.5.5 0 0 1-.5.5H3.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L3.707 7.5H9.5a.5.5 0 0 1 .5.5"/>
              </svg>
            </button>
          </Link>
        </div>
        <div className="col">
          <button type="button" id="save-and-reboot-button" className="btn pt-0 pb-0 btn-danger btn-outline-light h-100 w-100" onClick={saveAndReboot}>
            <div className="row">
              <div className="col ps-0 pe-0 d-flex align-items-center justify-content-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                     className="bi bi-sd-card" viewBox="0 0 16 16" style={{width: "3em", height: "3em"}}>
                  <path
                    d="M6.25 3.5a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0zm2 0a.75.75 0 0 0-1.5 0v2a.75.75 0 0 0 1.5 0z"/>
                  <path fillRule="evenodd"
                        d="M5.914 0H12.5A1.5 1.5 0 0 1 14 1.5v13a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 14.5V3.914c0-.398.158-.78.44-1.06L4.853.439A1.5 1.5 0 0 1 5.914 0M13 1.5a.5.5 0 0 0-.5-.5H5.914a.5.5 0 0 0-.353.146L3.146 3.561A.5.5 0 0 0 3 3.914V14.5a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5z"/>
                </svg>
              </div>
              <div className="col ps-0 pe-0 mt-0 mb-0 pt-0 pb-0">
                <span className="h4 weight-bold ">Save & &nbsp;</span>
                <span className="h4 weight-bold">Reboot</span>
              </div>
              <div className="col ps-0 pe-0 d-flex align-items-center justify-content-center">
                <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor"
                     className="bi bi-arrow-clockwise" viewBox="0 0 16 16" style={{width: "3em", height: "3em"}}>
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
                 viewBox="0 0 16 16" style={{ width: "3em", height: "3em" }}>
              <path
                d="M0 3.5A1.5 1.5 0 0 1 1.5 2h13A1.5 1.5 0 0 1 16 3.5v9a1.5 1.5 0 0 1-1.5 1.5h-13A1.5 1.5 0 0 1 0 12.5zM1.5 3a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5h13a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5z"/>
              <path
                d="M2 4.5a.5.5 0 0 1 .5-.5h3a.5.5 0 0 1 0 1H3v2.5a.5.5 0 0 1-1 0zm12 7a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1 0-1H13V8.5a.5.5 0 0 1 1 0z"/>
            </svg>
          </button>
        </div>
      </div>
      <div className="row ms-3 me-3">
        <h1 className="ps-0 mt-3 display-6">Known Networks:</h1>
        {hotspotSelector()}
      </div>
    </main>
  )
}

export default Settings;