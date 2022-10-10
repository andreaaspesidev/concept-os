import asyncio as aio
import contextlib


class ClearableQueue(aio.Queue):
    """
    Asyncio queue that can be cleared
    """

    def clear(self) -> None:
        """
        Blocking method to clear a Queue
        """
        while self.qsize() > 0:
            self.get_nowait()
            self.task_done()  # Always marks tasks as done


# this is a wrapper over the asyncio.wait that will cancel all tasks that are
# running after the 'await aio.wait' completes. Tasks that were cancelled will
# be returned in the 'pending' list (not 'done' list)
async def wait_cancel_others(fs, *args, **kwargs):
    # this may throw exceptions when cancelled
    try:
        # return the set of done and pending tasks
        return await aio.wait(fs, *args, **kwargs)
    # cleanup
    finally:
        # cancel all the tasks that are not done
        for task in [t for t in fs if not t.done()]:
            # cancel the task
            task.cancel()
            # this will throw cancelled error when we do the await
            with contextlib.suppress(aio.CancelledError):
                await task
