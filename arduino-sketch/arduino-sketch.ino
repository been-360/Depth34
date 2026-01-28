#include <Servo.h>

Servo rov_up1;

void setup() {
    rov_up1.attach(9);
    rov_up1.writeMicroseconds(1500);
    
    delay(5000);
}

void loop() {
    rov_up1.writeMicroseconds(1700);
    delay(20);
}
