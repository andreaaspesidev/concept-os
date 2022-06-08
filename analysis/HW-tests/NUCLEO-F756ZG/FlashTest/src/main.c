#include <stdio.h>
#include <string.h>
#include "main.h"

void UART3_Configuration(void);

UART_HandleTypeDef UART_Handler; /*Create UART_HandleTypeDef struct instance */
char Message[] = "Write t to start the flash test\r\n"; /* Message to be transmitted through UART */

uint32_t flash_test() {
    /* Unlock the Flash to enable the flash control register access *************/
    char buffer[100];
    uint32_t ticks;
    snprintf(buffer, 100, "Unlocking Flash...\n");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    HAL_FLASH_Unlock();
	  /* Erase the user Flash area */
    snprintf(buffer, 100, "Erasing sector 4...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    ticks = HAL_GetTick();
    FLASH_Erase_Sector(FLASH_SECTOR_4, VOLTAGE_RANGE_3);
    ticks = HAL_GetTick() - ticks;
    snprintf(buffer, 100, "%lu ms\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);

    /* Program half-word 0, setting the last bit -> 11111111_11111110*/
    uint32_t address = 0x08020000;
    snprintf(buffer, 100, "Programming bit 0...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    ticks = HAL_GetTick();
    if (HAL_FLASH_Program(FLASH_TYPEPROGRAM_HALFWORD, address, 0xFFFE) != HAL_OK) {
    	return HAL_FLASH_GetError();
    }
    ticks = HAL_GetTick() - ticks;
    /* Check the word */
    uint16_t word = *(__IO uint16_t *)address;
    if (word != 0xFFFE) {
    	return 10;
    }
    snprintf(buffer, 100, "ok! (%lu ms)\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);

    /* Change only one bit in half-word 0 -> 11111111_11111100*/
    snprintf(buffer, 100, "Programming bit 1...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    ticks = HAL_GetTick();
    if (HAL_FLASH_Program(FLASH_TYPEPROGRAM_HALFWORD, address, 0xFFFC) != HAL_OK) {
        return HAL_FLASH_GetError();
    }
    ticks = HAL_GetTick() - ticks;
    word = *(__IO uint16_t *)address;
    if (word != 0xFFFC) {
      return 10;
    }
    snprintf(buffer, 100, "ok! (%lu ms)\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    /* Change only one bit n half-word 0 -> 11111111_11111000*/
    snprintf(buffer, 100, "Programming bit 2...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    ticks = HAL_GetTick();
    if (HAL_FLASH_Program(FLASH_TYPEPROGRAM_HALFWORD, address, 0xFFF8) != HAL_OK) {
        return HAL_FLASH_GetError();
    }
    ticks = HAL_GetTick() - ticks;
    word = *(__IO uint16_t *)address;
    if (word != 0xFFF8) {
      return 10;
    }
    snprintf(buffer, 100, "ok! (%lu ms)\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);

    /* Programming the next half-word to 0x0000 */
    snprintf(buffer, 100, "Programming half word 1...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    /* Change only one bit in this word -> 11111111_11111000*/
    ticks = HAL_GetTick();
    if (HAL_FLASH_Program(FLASH_TYPEPROGRAM_HALFWORD, address+2, 0x0000) != HAL_OK) {
        return HAL_FLASH_GetError();
    }
    ticks = HAL_GetTick() - ticks;
    word = *(__IO uint16_t *)(address+2);
    if (word != 0x0000) {
      return 10;
    }
    snprintf(buffer, 100, "ok! (%lu ms)\nSuccess!\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    return 0;
}

int main(void)
{
  HAL_Init(); /* HAL library initialization */
  UART3_Configuration(); /* Call UART3 initialization define below */
  HAL_UART_Transmit(&UART_Handler, (uint8_t *)Message, strlen(Message), 10);
  while(1)
  {
     uint8_t buffer[1];
        HAL_UART_Receive(&UART_Handler, buffer, sizeof(buffer), HAL_MAX_DELAY);
        HAL_UART_Transmit(&UART_Handler, buffer, sizeof(buffer), HAL_MAX_DELAY);
        if (buffer[0] == 't') {
            flash_test();
        } 
  }
}

void UART3_Configuration(void)
{
  __HAL_RCC_GPIOD_CLK_ENABLE(); /* Enable clock to PORTA - UART2 pins PA2 and PA3 */
  __HAL_RCC_USART3_CLK_ENABLE(); /* Enable clock to UART2 module */
  
  GPIO_InitTypeDef UART3_GPIO_Handler; /*Create GPIO_InitTypeDef struct instance */
  UART3_GPIO_Handler.Pin = GPIO_PIN_8 | GPIO_PIN_9; 
  UART3_GPIO_Handler.Mode = GPIO_MODE_AF_PP;
  UART3_GPIO_Handler.Pull = GPIO_PULLUP;
  UART3_GPIO_Handler.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
  UART3_GPIO_Handler.Alternate = GPIO_AF7_USART3;
  HAL_GPIO_Init(GPIOD, &UART3_GPIO_Handler);
  //UART Configuration
  UART_Handler.Instance = USART3;
  UART_Handler.Init.BaudRate = 115200;
  UART_Handler.Init.Mode = UART_MODE_TX_RX;
  UART_Handler.Init.WordLength = UART_WORDLENGTH_8B;
  UART_Handler.Init.StopBits = UART_STOPBITS_1;
  UART_Handler.Init.OverSampling = UART_OVERSAMPLING_16;
  HAL_UART_Init(&UART_Handler);  
}

void SysTick_Handler(void)
{
  HAL_IncTick();
  HAL_SYSTICK_IRQHandler();
}