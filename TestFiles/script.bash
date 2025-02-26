ffmpeg -f v4l2 -input_format mjpeg -framerate 25 -video_size 1920x1080 -i /dev/video0 \
  -c:v libx264 -preset ultrafast -crf 5 -force_key_frames "expr:gte(t,n_forced*5)" \
  -f segment -reset_timestamps 1 -segment_time 5 -segment_wrap 5 output%03d.ts

