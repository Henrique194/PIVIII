#include "Controller.h"

void ArmServo::update_status(const char* msg) {
  if (!msg) {
    return;
  }

  if (msg == foward_msg) {
    move_foward = !move_foward;
    move_backward = false;
  } else if (msg == backward_msg) {
    move_backward = !move_backward;
    move_foward = false;
  }
}

void ArmServo::move() {
  Serial.println("Begin move");
  Serial.print("position = ");
  Serial.println(position);

  if (move_foward && position < 180) {
    position += 1;
  }
  if (move_backward && position > 0) {
    position -= 1;
  }

  servo.write(position);

  Serial.println("End move");
}

bool ArmServo::is_stopped() {
  return !(move_foward && position < 179)
         && !(move_backward && position > 1);
}

void ArmServo::set_position(uint8_t pos) {
  Serial.print("pos = ");
  Serial.println(pos);
  Serial.print("position = ");
  Serial.println(position);

  int16_t i = position;

  if (pos > position) {
    Serial.println("pos > position");
    for (; i <= pos && i <= 180; i++) {
      Serial.print("i = ");
      Serial.println(i);
      servo.write(i);
      delay(25);
    }
  }
  else {
    Serial.println("pos <= position");
    int16_t i = position;
    for (; i >= pos && i >= 0; i--) {
      Serial.print("i = ");
      Serial.println(i);
      servo.write(i);
      delay(25);
    }
  }

  if (i > 180) {
    position = 180;
  }
  else if(i < 0) {
    position = 0;
  }
  else {
    position = pos;
  }
}

Servo& ArmServo::get_servo() {
  return servo;
}
