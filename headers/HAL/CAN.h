/*----------------------------------------------------------------------------*/
/* Copyright (c) FIRST 2016-2017. All Rights Reserved.                        */
/* Open Source Software - may be modified and shared by FRC teams. The code   */
/* must be accompanied by the FIRST BSD license file in the root directory of */
/* the project.                                                               */
/*----------------------------------------------------------------------------*/

#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "../FRC_NetworkCommunication/CANSessionMux.h"

void canTxSend(uint32_t arbID, uint8_t length,
               int32_t period);

void canTxPackInt8(uint32_t arbID, uint8_t offset, uint8_t value);
void canTxPackInt16(uint32_t arbID, uint8_t offset, uint16_t value);
void canTxPackInt32(uint32_t arbID, uint8_t offset, uint32_t value);
void canTxPackFXP16(uint32_t arbID, uint8_t offset, double value);
void canTxPackFXP32(uint32_t arbID, uint8_t offset, double value);

uint8_t canTxUnpackInt8(uint32_t arbID, uint8_t offset);
uint32_t canTxUnpackInt32(uint32_t arbID, uint8_t offset);
uint16_t canTxUnpackInt16(uint32_t arbID, uint8_t offset);
double canTxUnpackFXP16(uint32_t arbID, uint8_t offset);
double canTxUnpackFXP32(uint32_t arbID, uint8_t offset);

bool canRxReceive(uint32_t arbID);

uint8_t canRxUnpackInt8(uint32_t arbID, uint8_t offset);
uint16_t canRxUnpackInt16(uint32_t arbID, uint8_t offset);
uint32_t canRxUnpackInt32(uint32_t arbID, uint8_t offset);
double canRxUnpackFXP16(uint32_t arbID, uint8_t offset);
double canRxUnpackFXP32(uint32_t arbID, uint8_t offset);
