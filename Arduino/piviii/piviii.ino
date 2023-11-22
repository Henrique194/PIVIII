#include "WString.h"
#include <Servo.h>
#include <string>


class ArmServo {
  Servo servo;
  bool move_foward;
  bool move_backward;
  std::string foward_msg;
  std::string backward_msg;
  uint8_t pin;
  uint8_t position;

public:
  ArmServo(uint8_t pin, const char* bwd_msg, const char* fwd_msg)
    : servo{}
    , move_foward{false}
    , move_backward{false}
    , foward_msg{ fwd_msg }
    , backward_msg{ bwd_msg }
    , pin{pin}
    , position{0}
  {
    servo.attach(pin);
    servo.write(0);
  }

  void update_status(const char* msg) {
    if (!msg) {
      return;
    }

    if (msg == foward_msg) {
      move_foward = !move_foward;
      move_backward = false;
    }
    else if (msg == backward_msg) {
      move_backward = !move_backward;
      move_foward = false;
    }

    move();
  }

  void move() {
    if (move_foward && position < 179) {
      position += 2;
    }
    if (move_backward && position > 1) {
      position -= 2;
    }

    servo.write(position);
  }
};

std::vector<ArmServo> servos {
  ArmServo(D5, "rightshoulder", "leftshoulder"),
  ArmServo(D6, "back", "start"),
  ArmServo(D7, "b", "a")
};

void setup() {
  Serial.begin(115200);
  pinMode(D5, OUTPUT);
}

void loop() {
  if (!Serial.available()) {
    for (auto& servo : servos) {
      servo.move();
    }
    return;
  }

  String serial_str = Serial.readString();
  char* str = (char*) malloc(serial_str.length() + 1);
  strcpy(str, serial_str.c_str());

  char* token = strtok(str, "/");

  while (token) {
    for (auto& servo : servos) {
      servo.update_status(token);
    }
    token = strtok(NULL, "/");
  }

  free(str);
  str = NULL;

  delay(15);
}
