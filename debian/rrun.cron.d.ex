#
# Regular cron jobs for the rrun package
#
0 4	* * *	root	[ -x /usr/bin/rrun_maintenance ] && /usr/bin/rrun_maintenance
