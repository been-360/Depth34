void setup() {
  Serial.begin(115200);
  while (!Serial);
  Serial.println("Setup Complete");
}

void loop() {
  if (Serial.available() > 0) {
    String received = Serial.readStringUntil('\n');
    
    Serial.print("Received: ");
    Serial.println(received);
  }
}
