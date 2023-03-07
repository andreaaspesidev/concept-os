# The easiest way to get ggplot2 is to install the whole tidyverse:
# install.packages("tidyverse")

library(data.table)
library(ggplot2)
library(tibble)
library(dplyr)
library(tidyr)

# Set working directory
this.dir <- dirname(parent.frame(2)$ofile)
setwd(this.dir)

# Import dataset
unavailability <- read.csv("data/conceptos-component-unavailability.csv", header = TRUE, sep = ",")
# Convert to microseconds
unavailability[2] <- unavailability[2] * 10^6
# Rename elements
unavailability[1] <- c("(0) RCC","(15) UART", "(25) STORAGE", 
"(30) UPDATE","(255) IDLE","(10) TICKER",
"(5) BTHERMO","(20) BTHERMO_CTRL")

# Plot max timings
graph <- ggplot(data = unavailability, aes(
  x = forcats::fct_rev(factor(component_id, level=c(
    "(0) RCC","(5) BTHERMO","(10) TICKER","(15) UART", 
    "(20) BTHERMO_CTRL","(25) STORAGE", 
    "(30) UPDATE","(255) IDLE"))),
  y = unavailable_time,
)) +
  geom_bar(aes(y = unavailable_time),
    stat = "identity",
    position = position_dodge()
  ) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y = expression(paste("Max component unavailability [", mu, "s]")), x = "Component")
  
graph

ggsave(
  "output/component_unavailability.pdf",
  plot = graph,
  scale = 1,
  width = 1500, height = 500, units = "px",
  dpi = 300,
)
