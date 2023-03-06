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
resources <- read.csv("data/resources.csv", header = TRUE, sep = ",")

# Rename columns
colnames(resources) <- c("os_type","Total Flash", "Total SRAM", "Kernel Flash", "Kernel SRAM", "Update-required Flash")

# Promote columns to rows
df1 <- resources %>% 
  gather(type, value, -os_type) %>%
  arrange(type)

# Change names
df1$os_type[df1$os_type == "conceptos"] <- "ConceptOS"
df1$os_type[df1$os_type == "hubris"] <- "Hubris"

# Plot
resource_comparison <- ggplot(data = df1, aes(
  x = forcats::fct_rev(type),
  y = value,
  fill = os_type
)) +
  geom_bar(aes(y = value),
           stat = "identity",
           position = position_dodge()
  ) +
  scale_y_continuous(minor_breaks = seq(0, 120000, by = 5000)) + 
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y = "Used space [bytes]", x = "Resource") +
  theme(
    legend.title = element_blank(),
    legend.position = c(0.8, 0.85),
    legend.box = "horizontal",
    plot.margin = margin(0.1,0.5,0.1,0.1, "cm")
  ) + 
  guides(fill = guide_legend(reverse = TRUE))

resource_comparison

ggsave(
  "output/resources.pdf",
  plot = resource_comparison,
  scale = 1,
  width = 1920, height = 1080, units = "px",
  dpi = 300,
)