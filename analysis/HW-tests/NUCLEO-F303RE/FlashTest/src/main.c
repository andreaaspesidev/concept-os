//#include "stm32f3xx_hal_flash.h"
#include <stdio.h>
#include <string.h>
#include "main.h"

void UART2_Configuration(void);

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
    snprintf(buffer, 100, "Erasing sector 16...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    
    FLASH_EraseInitTypeDef InitEraseStruct;
    InitEraseStruct.TypeErase=FLASH_TYPEERASE_PAGES;
    InitEraseStruct.PageAddress=ADDR_FLASH_PAGE_16;
    InitEraseStruct.NbPages=1;
    uint32_t PageError;
    ticks = HAL_GetTick();
    HAL_FLASHEx_Erase(&InitEraseStruct, &PageError);
    ticks = HAL_GetTick() - ticks;
    snprintf(buffer, 100, "%lu ms\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);

    /* Program first half-word, setting the first bit -> 11111111_11111110*/
    uint32_t address = ADDR_FLASH_PAGE_16;
    snprintf(buffer, 100, "Programming half word 0...");
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

    /* Program second half-word, setting the second bit -> 11111111_11111101*/
    word = *(__IO uint16_t *)(address+2);
    if (word != 0xFFFF) {
    	return 10;
    }
    snprintf(buffer, 100, "Programming half word 1...");
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    ticks = HAL_GetTick();
    if (HAL_FLASH_Program(FLASH_TYPEPROGRAM_HALFWORD, address+2, 0xFFFD) != HAL_OK) {
        return HAL_FLASH_GetError();
    }
    ticks = HAL_GetTick() - ticks;
    word = *(__IO uint16_t *)(address+2);
    if (word != 0xFFFD) {
      return 10;
    }
    snprintf(buffer, 100, "ok! (%lu ms)\nSuccess!\n", ticks);
    HAL_UART_Transmit(&UART_Handler, (uint8_t*)buffer, strlen(buffer), HAL_MAX_DELAY);
    return 0;
}

int main(void)
{
  HAL_Init(); /* HAL library initialization */
  UART2_Configuration(); /* Call UART2 initialization define below */
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

void UART2_Configuration(void)
{
  __HAL_RCC_GPIOA_CLK_ENABLE(); /* Enable clock to PORTA - UART2 pins PA2 and PA3 */
  __HAL_RCC_USART2_CLK_ENABLE(); /* Enable clock to UART2 module */
  
  GPIO_InitTypeDef UART2_GPIO_Handler; /*Create GPIO_InitTypeDef struct instance */
  UART2_GPIO_Handler.Pin = GPIO_PIN_2 | GPIO_PIN_3; 
  UART2_GPIO_Handler.Mode = GPIO_MODE_AF_PP;
  UART2_GPIO_Handler.Pull = GPIO_PULLUP;
  UART2_GPIO_Handler.Speed = GPIO_SPEED_FREQ_HIGH;
  UART2_GPIO_Handler.Alternate = GPIO_AF7_USART2;
  HAL_GPIO_Init(GPIOA, &UART2_GPIO_Handler);
  //UART Configuration
  UART_Handler.Instance = USART2;
  UART_Handler.Init.BaudRate = 38400;
  UART_Handler.Init.WordLength = UART_WORDLENGTH_8B;
  UART_Handler.Init.StopBits = UART_STOPBITS_1;
  UART_Handler.Init.Parity = UART_PARITY_NONE;
  UART_Handler.Init.Mode = UART_MODE_TX_RX;
  UART_Handler.Init.HwFlowCtl = UART_HWCONTROL_NONE;
  UART_Handler.Init.OverSampling = UART_OVERSAMPLING_16;
  UART_Handler.Init.OneBitSampling = UART_ONE_BIT_SAMPLE_DISABLE;
  UART_Handler.AdvancedInit.AdvFeatureInit = UART_ADVFEATURE_NO_INIT;
  HAL_UART_Init(&UART_Handler);  
}

void SysTick_Handler(void)
{
  HAL_IncTick();
  HAL_SYSTICK_IRQHandler();
}