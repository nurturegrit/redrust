I will be documenting the development of this Redis Server Clone in this file. The Development will be divided into several steps which will be reflected in the commit history.


## Step 1 -> Setting up TCP Port Connection

To Do:
- Set up a TCP Port connection at local host with a `user provided host` when available else `6379` port as default. Port should be able to handle:
    * multiple connections
    * concurrent requests
    * Errors
    * Graceful shutdown