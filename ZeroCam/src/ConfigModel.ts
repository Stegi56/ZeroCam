export interface Config {
  telegram_key          : string;
  camera_input          : CameraInput;
  motion_listener       : MotionListener;
  gui_stream_output     : GUIStreamOutput;
  internet_stream_output: InternetStreamOutput;
  g_cloud               : GCloud;
  hotspot_networks      : Array<string>;
}

interface CameraInput {
  resolution: string;
  fps       : string;
  clip      : Clip;
}

interface Clip {
  segment_size_sec     : number;
  segments             : number;
  timer_before_clip_sec: number;
  cooldown_sec         : number;
  disk_full_buffer_gb  : number;
}

interface MotionListener {
  sensitivity_inverse : number;
  threshold_sum_kilo  : number;
  frame_delay_millisec: number;
  trigger_duration    : number;
}

interface GUIStreamOutput {
  resolution: string;
  bit_rate  : string;
  fps       : string;
}

interface InternetStreamOutput {
  url       : string;
  username  : string;
  password  : string;
  resolution: string;
  bit_rate  : string;
  fps       : string;
}

interface GCloud {
  limit_gb                    : number;
  backup_scheduler_timeout_sec: number;
}



