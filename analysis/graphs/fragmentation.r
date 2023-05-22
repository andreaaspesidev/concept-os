# The easiest way to get ggplot2 is to install the whole tidyverse:
# install.packages("tidyverse")

library(data.table)
library(ggplot2)
library(tibble)
library(dplyr)
library(tidyr)
library(ggpubr)

# Set working directory
this.dir <- dirname(parent.frame(2)$ofile)
setwd(this.dir)

# Import dataset
hubris <- read.csv("data/hubris-component-sizes.csv",header=TRUE, sep= ",", row.names = 1)
conceptos <- read.csv("data/conceptos-component-sizes.csv",header=TRUE, sep= ",", row.names = 1)

# Remove components/data not useful for the comparison
# conceptos <- conceptos %>% select(-c("STORAGE","Unused"))
conceptos <- conceptos %>% select(-c("Unused"))
hubris <- hubris %>% select(-c("Unused"))
# Add a fictitious STORAGE in hubris for the graph to show correctly
hubris['STORAGE'] = c(0,0)

# Compute fragmentation
conceptos[nrow(conceptos) + 1,] = diff(as.matrix(conceptos))
rownames(conceptos) <- list("used","total","fragmentation")
hubris[nrow(hubris) + 1,] = diff(as.matrix(hubris))
rownames(hubris) <- list("used","total","fragmentation")

# Extract the usage
used_resources <- rbind(
  subset(conceptos, subset=rownames(conceptos) == 'used'),
  subset(hubris, subset=rownames(hubris) == 'used')
)
rownames(used_resources) <- list("ConceptOS","Hubris")

# Switch rows with columns
df1 <- transpose(used_resources)
colnames(df1) <- rownames(used_resources)
rownames(df1) <- colnames(used_resources)
df2 <- tibble::rownames_to_column(df1, "component")
used_resources <- df2 %>% 
  gather(os_type, used, -component) %>%
  arrange(component)

used_graph <- ggplot(data = used_resources, aes(x = forcats::fct_rev(component),
                                           y = used,
                                           fill = os_type)) +
  geom_bar(
    aes(y = used),
    stat = "identity",
    position = position_dodge()) +
  scale_y_continuous(breaks = seq(0, 8192, by = 2048)) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y ="Used flash", x = "") +
  theme(
    legend.title = element_blank(),
    legend.box = "horizontal"
  ) + 
  guides(fill = guide_legend(reverse = TRUE))


# Extract the total size
total_resources <- rbind(
  subset(conceptos, subset=rownames(conceptos) == 'total'),
  subset(hubris, subset=rownames(hubris) == 'total')
)
rownames(total_resources) <- list("ConceptOS","Hubris")

# Switch rows with columns
df1 <- transpose(total_resources)
colnames(df1) <- rownames(total_resources)
rownames(df1) <- colnames(total_resources)
df2 <- tibble::rownames_to_column(df1, "component")
total_resources <- df2 %>% 
  gather(os_type, total, -component) %>%
  arrange(component)

total_graph <- ggplot(data = total_resources, aes(x = forcats::fct_rev(component),
                                           y = total,
                                           fill = os_type)) +
  geom_bar(
    aes(y = total),
    stat = "identity",
    position = position_dodge()) +
  scale_y_continuous(limits = c(0, 8192), breaks = seq(0, 8192, by = 1024)) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y ="Total allocated flash", x = "") + 
  theme(
    legend.title = element_blank(),
    legend.box = "horizontal"
  ) + 
  guides(fill = guide_legend(reverse = TRUE))


# Extract the fragmentation
frag_resources <- rbind(
  subset(conceptos, subset=rownames(conceptos) == 'fragmentation'),
  subset(hubris, subset=rownames(hubris) == 'fragmentation')
)
rownames(frag_resources) <- list("ConceptOS","Hubris")

# Switch rows with columns
df1 <- transpose(frag_resources)
colnames(df1) <- rownames(frag_resources)
rownames(df1) <- colnames(frag_resources)
df2 <- tibble::rownames_to_column(df1, "component")
frag_resources <- df2 %>% 
  gather(os_type, fragmentation, -component) %>%
  arrange(component)

frag_graph <- ggplot(data = frag_resources, aes(x = forcats::fct_rev(component),
                                                y = fragmentation,
                                                fill = os_type)) +
  geom_bar(
    aes(y = fragmentation),
    stat = "identity",
    position = position_dodge()) +
  scale_y_continuous(breaks = seq(0, 4096, by = 1024)) +
  coord_flip() +
  scale_fill_grey() +
  theme_bw() +
  labs(y ="Unused flash\n(fragmentation)", x = "") +
  theme(
    legend.title = element_blank(),
    legend.box = "horizontal"
  ) + 
  guides(fill = guide_legend(reverse = TRUE))


plot <- ggarrange(total_graph, ggarrange(used_graph,frag_graph, ncol = 2, labels = c("B","C"),legend = "none"), labels = "A", nrow = 2, common.legend = TRUE, legend="bottom")

plot

ggsave(
  "output/component-sizes.pdf",
  plot = plot,
  scale = 1,
  width = 1920, height = 1400, units = "px",
  dpi = 300,
)