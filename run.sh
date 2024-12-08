PORT=7878


cargo run --bin qps_client  -- \
    --model-name "resnet"\
    --url "localhost:$PORT"\
    --queue-capacity 1000\
    --producer-num 1\
    --consumer-num 1\
    --batch-size 1\
    --connect-num 1\
    --measure-type quick