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
unavailability <- read.csv("data/system-unavailability.csv", header = TRUE, sep = ",")

unavailability[1] <- c("ConceptOS", "Hubris")

# Plot max timings
graph <- ggplot(data = unavailability, aes(
  x = forcats::fct_rev(os_type),
  y = unavailable_time,
)) +
  geom_bar(aes(y = unavailable_time),
    stat = "identity",
    position = position_dodge()
  ) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y = "Max system unavailability [ms]", x = "Operating System")
  
graph

ggsave(
  "output/system_unavailability.pdf",
  plot = graph,
  scale = 1,
  width = 1500, height = 500, units = "px",
  dpi = 300,
)
