from application_client.client import Nearbackend


# In this test we check that the get_version replies the right application version
def test_get_version_cmd(backend):
    # Use the app interface instead of raw interface
    client = Nearbackend(backend)
    # Send the get_version instruction to the app
    version = client.get_version().data
    assert len(version) == 3
    # Assert that we have received the correct app version compared as Makefile data
    assert (version[0], version[1], version[2]) == (2, 2, 1)
