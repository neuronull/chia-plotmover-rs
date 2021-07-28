# chia-plotmover-rs
Replaces legacy Chia plots with newly created (NFT) plots, while you are plotting

# Notes

Use this tool to incrementally replace your legacy Chia plots with new ones that have pool support.

Why create this ? Well, efficiency. Its a waste to have to delete an entire drive worth of plots. Time that could be spent farming. And its a waste of time to have to manually remove plots periodically, when we can have software do that for us.

Tested on Ubuntu 20.04 , should work on any linux distro. Untested on Mac / Windows, use at your own risk. Try it out with dummy files and let me know how it goes.

How it works:
 - on boot, if any plots exist in the ssds , it moves them to destination hdds.
 - to move to a destination hdd, first scan the hdds list for available space. If space exists, just use that drive. If no space exists, delete a legacy plot from one of the drives, and place the new plot on that drive
 - monitors the filesystem for changes to the ssds dirs, and if new plots appear, moves them to hdd using the same procedure mentioned in the previous bullet.

Please fully read the usage and understand the directory structure required to use.
If the directories don't match the config file, and the legacy_plots / pool_plots on the hdds , then the program will fail.

This program does remove plots from your drive, that is its intent.
But it does so only when new plots are available to take the space freed by removing the old plot.

Additionally, it does not rely on static sleep but instead is notified of filesystem events when a new plot is created.

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

# Example

In the below example, there is one new pool plot which is already done on the ssd and ready to be moved to an hdd.
There was also space available on hdd3, so we didn't need to remove a plot.
Then, we notice that hdd1 and hdd2 have no legacy plots left, so we ignore them from here now.
When the next plot is complete, we remove a legacy plot from hdd3 and copied the new plot to the hdd3, then removed the new plot from the ssd when that succeeded.


```
$ ./target/debug/plotmover-rs 
2021-07-27 20:21:17 {INFO} checking for plot files in path /mnt/ssd1/plots
2021-07-27 20:21:17 {INFO} checking for plot files in path /mnt/ssd2/plots
2021-07-27 20:21:17 {INFO} hdd "/mnt/hdd3" has space, using that
2021-07-27 20:21:18 {INFO} copy plot "/mnt/ssd2/plots/plot-k32-2021-07-27-18-12-<pool-plot>.plot" to "/mnt/hdd3/pool_plots/plot-k32-2021-07-27-18-12-<pool-plot>.plot" ... 
2021-07-27 20:28:15 {INFO}  ... completed
2021-07-27 20:28:17 {INFO} monitoring these dirs for new plots ["/mnt/ssd1/plots", "/mnt/ssd2/plots"]
2021-07-27 20:56:49 {INFO} checking for plot files in path /mnt/ssd1/plots
2021-07-27 20:56:50 {INFO} removed plot "/mnt/hdd3/legacy_plots/plot-k32-2021-05-29-22-39-<legacy_plot>.plot"
2021-07-27 20:56:50 {INFO} no plots to remove in path /mnt/hdd1, removing it from the list
2021-07-27 20:56:50 {INFO} no plots to remove in path /mnt/hdd2, removing it from the list
2021-07-27 20:56:50 {INFO} copy plot "/mnt/ssd1/plots/plot-k32-2021-07-27-20-21-<pool_plot>.plot" to "/mnt/hdd3/pool_plots/plot-k32-2021-07-27-20-21-<pool_plot>.plot" ... 
2021-07-27 21:05:00 {INFO}  ... completed
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd1/plots
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd1/plots
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd2/plots
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd2/plots
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd2/plots
2021-07-27 21:05:02 {INFO} checking for plot files in path /mnt/ssd1/plots
2021-07-27 21:32:34 {INFO} checking for plot files in path /mnt/ssd2/plots
```

# Support

XCH: xch1ahj4fpvy5a9jreg03k5hg7s7zau6vndns2u8mav37r4sez6uafqq86r6rx

ETH-ERC20: 0x88Ef3125e6C5520487731D247b7aeBc78C62CF20

LTC: ltc1qxnwz7rfwk6e56dn3umfeg6g9gxeyfh2cf6vqjl

BTC: bc1q7huxnpqf93ejh03y5w0gpxrrrrdfphssd0hdle
