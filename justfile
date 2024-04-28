test day part : 
  cd day-{{day}} && cargo test --bin p{{part}} -- --nocapture


work day part : 
  cd day-{{day}} && cargo run --bin p{{part}}
