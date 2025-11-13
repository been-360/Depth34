#include <USBHost.h>
#include <Servo.h>

// all the numbers are from the other RPI script so it might need tuning

USBHost myusb;
HIDJoystick joystick(myusb);

#define ROVUP1 17
#define ROVUP2 27
#define ROVM1 5
#define ROVM2 6
#define ROVM3 19
#define ROVM4 26
#define ARMPIN1 9
#define ARMPIN2 10
#define HANDCODE 11

Servo rovUp1, rovUp2, rovM1, rovM2, rovM3, rovM4, arm1, arm2, hand;

typedef struct {
  int rightjoyX;
  int rightjoyY;
  int leftjoyX;
  int leftjoyY;
  int lefttrigger;
  int righttrigger;
  bool Ybutton;
  bool Xbutton;
  bool Bbutton;
  bool Abutton;
  bool Backbutton;
  bool Startbutton;
  bool Dpadup;
  bool Dpadleft;
  bool Dpadright;
  bool Dpaddown;
} controller;

controller logi;

int scale_axis(int value) {
  return (int)(value * 500.0 / 32767.0);
}

int clamp_servo(int value) {
  if (value < 500) return 500;
  if (value > 2500) return 2500;
  return value;
}

void servomove(controller logi) {
  int lx = scale_axis(logi.leftjoyX);
  int ly = scale_axis(logi.leftjoyY);
  int rx = scale_axis(logi.rightjoyX);
  int ry = scale_axis(logi.rightjoyY);
  int lt = scale_axis(logi.lefttrigger);
  int rt = scale_axis(logi.righttrigger);
  int boost = (1 + logi.Startbutton);


  // I DIRECTLY GRABBED THIS MATH FROM THE OLD CODE AND HAVE NO IDEA HOW IT WORKS, SO IT MIGHT BE WRONG

  rovUp1.writeMicroseconds(clamp_servo(1500 + 100 * logi.Dpadup * boost - 100 * logi.Dpaddown * boost + 50 * logi.Bbutton - 50 * logi.Xbutton));
  rovUp2.writeMicroseconds(clamp_servo(1500 + 100 * logi.Dpadup * boost - 100 * logi.Dpaddown * boost + 50 * logi.Xbutton - 50 * logi.Bbutton));

  rovM1.writeMicroseconds(clamp_servo(1500 + lx * boost + ly * boost + 50 * logi.Ybutton - 50 * logi.Abutton));
  rovM2.writeMicroseconds(clamp_servo(1500 + lx * boost - ly * boost + 50 * logi.Ybutton - 50 * logi.Abutton));
  rovM3.writeMicroseconds(clamp_servo(1500 - lx * boost + ly * boost + 50 * logi.Ybutton - 50 * logi.Abutton));
  rovM4.writeMicroseconds(clamp_servo(1500 - lx * boost - ly * boost + 50 * logi.Ybutton - 50 * logi.Abutton));

  hand.writeMicroseconds(clamp_servo(1500 + lt * boost - rt * boost));
  arm1.writeMicroseconds(clamp_servo(1500 + ry * boost));
  arm2.writeMicroseconds(clamp_servo(1500 + rx * boost));
}

// MARK: SETUP

void setup() {
  Serial.begin(115200);
  myusb.begin();
  Serial.println("Where the joystick at?");

  rovUp1.attach(ROVUP1);
  rovUp2.attach(ROVUP2);
  rovM1.attach(ROVM1);
  rovM2.attach(ROVM2);
  rovM3.attach(ROVM3);
  rovM4.attach(ROVM4);
  arm1.attach(ARMPIN1);
  arm2.attach(ARMPIN2);
  hand.attach(HANDCODE);
}

// MARK: LOOP

void loop() {
  myusb.Task();

  if (joystick.available()) {
    logi.leftjoyX = joystick.getX();
    logi.leftjoyY = joystick.getY();
    logi.rightjoyX = joystick.getRx();
    logi.rightjoyY = joystick.getRy(); 
    logi.lefttrigger = joystick.getZ();
    logi.righttrigger = joystick.getRz();

    logi.Ybutton = joystick.getButton(4);
    logi.Xbutton = joystick.getButton(3);
    logi.Bbutton = joystick.getButton(2);
    logi.Abutton = joystick.getButton(1);
    logi.Backbutton = joystick.getButton(7);
    logi.Startbutton = joystick.getButton(8);

    uint8_t hat = joystick.getHatSwitch();
    logi.Dpadup = (hat == 0);
    logi.Dpadright = (hat == 2);
    logi.Dpaddown = (hat == 4);
    logi.Dpadleft = (hat == 6);

    servomove(logi);

    Serial.print("LX: "); Serial.print(logi.leftjoyX);
    Serial.print("  LY: "); Serial.print(logi.leftjoyY);
    Serial.print("  RX: "); Serial.print(logi.rightjoyX);
    Serial.print("  RY: "); Serial.print(logi.rightjoyY);
    Serial.print("  Hat: "); Serial.println(hat);
  }

  delay(50);
}
