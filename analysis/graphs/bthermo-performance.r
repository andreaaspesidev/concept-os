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
hubris <- read.csv("data/hubris-bthermo-performance.csv",header=TRUE, sep= ",", row.names = 1)
conceptos <- read.csv("data/conceptos-bthermo-performance.csv",header=TRUE, sep= ",", row.names = 1)

# colnames(hubris) <- c("(0) SEND", "(1) RECV", "(2) REPLY", "(3) SET_TIMER", "(4) BORROW_READ", "(5) BORROW_WRITE", "(6) BORROW_INFO", "(7) IRQ_CONTROL", "(9) GET_TIMER")
# colnames(conceptos) <- c("(0) SEND", "(1) RECV", "(2) REPLY", "(3) SET_TIMER", "(4) BORROW_READ", "(5) BORROW_WRITE", "(6) BORROW_INFO", "(7) IRQ_CONTROL", "(9) GET_TIMER")

# Extract max values
max_timings <- rbind(
  subset(conceptos, subset=rownames(conceptos) == 'MAX') * 10^6,
  subset(hubris, subset=rownames(hubris) == 'MAX') * 10^6
)
rownames(max_timings) <- list("ConceptOS","Hubris")
# Multiply type 1 for 100
max_timings$type1 <- max_timings$type1 * 100
# Change row names
colnames(max_timings) <- c("Phase #1\n(x100)", "Phase #2", "Phase #3", "Phase #4")

df1 <- transpose(max_timings)
colnames(df1) <- rownames(max_timings)
rownames(df1) <- colnames(max_timings)
df2 <- tibble::rownames_to_column(df1, "cycle_type")
df3 <- df2 %>% 
  gather(os_type, max, -cycle_type) %>%
  arrange(cycle_type)
# Plot max timings
max_graph <- ggplot(data = df3,aes(x = forcats::fct_rev(cycle_type),
                                   y = max,
                                   fill = os_type)) +
  geom_bar(aes(y=max),
           stat = "identity",
           position = position_dodge()) +
  scale_y_continuous(breaks = seq(0, 500, by = 50)) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y =expression(paste("Max execution time [", mu, "s]")), x = "Execution Phase") +
  theme(
    legend.title = element_blank(),
    legend.position = c(0.8, 0.8),
    legend.box = "horizontal"
  ) + 
  guides(fill = guide_legend(reverse = TRUE))

ggsave(
  "output/bthermo_performance_max_timings.pdf",
  plot = max_graph,
  scale = 1,
  width = 1920, height = 1080, units = "px",
  dpi = 300,
)

avg_timings <- rbind(
  subset(conceptos, subset=rownames(conceptos) == 'AVG') * 10^6,
  subset(hubris, subset=rownames(hubris) == 'AVG') * 10^6,
  subset(conceptos, subset=rownames(conceptos) == 'AVG_CONF_MIN') * 10^6,
  subset(hubris, subset=rownames(hubris) == 'AVG_CONF_MIN') * 10^6,
  subset(conceptos, subset=rownames(conceptos) == 'AVG_CONF_MAX') * 10^6,
  subset(hubris, subset=rownames(hubris) == 'AVG_CONF_MAX') * 10^6
)
rownames(avg_timings) <- list("cos_avg","h_avg","cos_avg_min","h_avg_min","cos_avg_max","h_avg_max")
# Multiply type 1 for 100
avg_timings$type1 <- avg_timings$type1 * 100
# Change row names
colnames(avg_timings) <- c("Phase #1\n(x100)", "Phase #2", "Phase #3", "Phase #4")
df1 <- transpose(avg_timings)
colnames(df1) <- rownames(avg_timings)
rownames(df1) <- colnames(avg_timings)
df2 <- tibble::rownames_to_column(df1, "cycle_type")
# Some data manupulation
df3 <- df2 %>% 
  gather(os_type, avg, -cycle_type,-cos_avg_min,-h_avg_min,-cos_avg_max,-h_avg_max) %>%
  subset(select=-c(2,3,4,5)) %>%
  arrange(cycle_type)

df3$os_type[df3$os_type == "cos_avg"] <- "ConceptOS"
df3$os_type[df3$os_type == "h_avg"] <- "Hubris"

df4 <- df2 %>% 
  gather(os_type, avg_min, -cycle_type,-cos_avg,-h_avg,-cos_avg_max,-h_avg_max) %>%
  subset(select=-c(2,3,4,5)) %>%
  arrange(cycle_type)

df4$os_type[df4$os_type == "cos_avg_min"] <- "ConceptOS"
df4$os_type[df4$os_type == "h_avg_min"] <- "Hubris"

df5 <- df2 %>% 
  gather(os_type, avg_max, -cycle_type,-cos_avg,-h_avg,-cos_avg_min,-h_avg_min) %>%
  subset(select=-c(2,3,4,5)) %>%
  arrange(cycle_type)

df5$os_type[df5$os_type == "cos_avg_max"] <- "ConceptOS"
df5$os_type[df5$os_type == "h_avg_max"] <- "Hubris"

df6 <- df3 %>% 
  inner_join(df4, by=c("os_type", "cycle_type"))  %>% 
  inner_join(df5, by=c("os_type", "cycle_type"))

# Plot avg with confidence intervals
avg_graph <- ggplot(df6, aes(x = forcats::fct_rev(cycle_type),
                             y = avg,
                             ymin = avg_min,
                             ymax = avg_max,
                             fill = os_type)) +
  geom_bar(aes(y=avg),stat = "identity",
           position = position_dodge()) +
  geom_errorbar( 
    width=0.2, 
    position = position_dodge(width=0.9),
    alpha=0.9) + 
  scale_y_continuous(breaks = seq(0, 400, by = 50)) + 
  coord_flip() +
  scale_fill_grey() +
  theme_bw() + 
  labs(y =expression(paste("Average execution time [", mu, "s]")), x = "Execution Phase") +
  theme(
    legend.title = element_blank(), 
    legend.position = c(0.8, 0.8), 
    legend.box = "horizontal"
  ) + 
  guides(fill = guide_legend(reverse = TRUE))

ggsave(
  "output/bthermo_performance_avg_timings.pdf",
  plot = avg_graph,
  scale = 1,
  width = 1920, height = 1080, units = "px",
  dpi = 300,
)

# Create a composed graph
plot <- ggarrange(max_graph, avg_graph, labels = c("A","B"), nrow = 2, common.legend = TRUE, legend="bottom")

ggsave(
  "output/bthermo_performance_recap.pdf",
  plot = plot,
  scale = 1,
  width = 1920, height = 1400, units = "px",
  dpi = 300,
)