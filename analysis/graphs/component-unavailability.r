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

# Plot max timings
graph <- ggplot(data = unavailability, aes(
  x = forcats::fct_rev(factor(component_id, level=c(
    "(2) RCC","(3) UART", "(4) STORAGE", 
    "(5) UPDATE","(8) IDLE","(9) TICKER",
    "(10) BTHERMO","(11) BTHERMO_CTRL"))),
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
