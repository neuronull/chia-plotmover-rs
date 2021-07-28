# chia-plotmover-rs
Replaces legacy Chia plots with newly created (NFT) plots, while you are plotting

# Notes

Use this tool to incrementally replace your legacy Chia plots with new ones that have pool support.

Tested on Ubuntu 20.04 , should work on any linux distro. Untested on Mac / Windows, use at your own risk. Try it out with dummy files and let me know how it goes.

How it works:
 - on boot, if any plots exist in the ssds , it moves them to destination hdds.
 - to move to a destination hdd, first scan the hdds list for available space. If space exists, just use that drive. If no space exists, delete a legacy plot from one of the drives, and place the new plot on that drive
 - monitors the filesystem for changes to the ssds dirs, and if new plots appear, moves them to hdd using the same procedure mentioned in the previous bullet.

Please fully read the usage and understand the directory structure required to use.
If the directories don't match the config file, and the legacy_plots / pool_plots on the hdds , then the program will fail.

This program does remove plots from your drive, that is its intent.
But it does so only when new plots are available to take the space freed by removing the old plot.

Why create this ? Well, efficiency. Its a waste to have to delete an entire drive worth of plots. Time that could be spent farming. And its a waste of time to have to manually remove plots periodically, when we can have software do that for us.

# Usage

This is the config file:

```
[dirs]
ssds = ["/mnt/ssd1/plots", "/mnt/ssd2/plots"]

hdds = [
    "/mnt/hdd1",
    "/mnt/hdd2",
    "/mnt/hdd3",
    "/mnt/hdd4",
    "/mnt/hdd5",
    "/mnt/hdd6",
    "/mnt/hdd7",
    "/mnt/hdd8",
]
```

Populate ssds with a list of the directories where new plots are to be found. These are the plots you want to keep.
I am having success with madmax plotter, alternating between two SSDs.

hdds is a list of your final plot destinations and also the drives where your legacy plots currently live.

*NOTE* The following directory structure should exist on your hdds:

/mnt/hdd1/legacy_plots
/mnt/hdd1/pool_plots

legacy_plots contains well, the legacy plots.
pool_plots is where the chia-plotmover-rs will move the new plots to.
