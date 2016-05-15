var searchIndex = {};
searchIndex["filedes"] = {"doc":"The top-level module filedes contains convenience / test stuff for playing with file descriptors.","items":[[5,"setup","filedes","",null,{"inputs":[],"output":{"name":"result"}}],[5,"teardown","","",null,{"inputs":[],"output":{"name":"result"}}],[5,"make_socket_addr","","",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"server_socket","","",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"connect_to_socket","","",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"unix_socket_pair","","Creates a socketpair in the UNIX domain and returns it.",null,{"inputs":[],"output":{"name":"result"}}],[5,"add_two_sockets_to_ring","","Creates a new pair of sockets with {unix_socket_pair} and adds it\nto the ring, and returns the number of sockets added.  This is the\neasiest way to get throwaway file descriptor outside wrapping\n`libc::mkstemp` (see [`add_tmpfile_to_ring`](fn.add_tmpfile_to_ring.html) for that) (:",null,{"inputs":[{"name":"ring"}],"output":{"name":"result"}}],[5,"add_tmpfile_to_ring","","Uses `mkstemp` to create an open, unlinked temporary file and add\nits file descriptor to the [`Ring`](ring/struct.Ring.html)\nstructure.",null,{"inputs":[{"name":"ring"}],"output":{"name":"result"}}],[0,"ring","","",null,null],[3,"Ring","filedes::ring","A ring buffer containing file descriptors.",null,null],[12,"count","","The number of file descriptors contained in the ring buffer.",0,null],[3,"RingIter","","An iterator over the File descriptors contained in an FD ring buffer",null,null],[4,"ProtocolError","","Any sort of error that can occur while trying to speak the ring\nbuffer protocol",null,null],[13,"NoFDReceived","","Expected to receive an FD, but did not get one",1,null],[13,"RingFormatError","","Something approximating a Ring was sent over the socket, but the number format didn&#39;t parse",1,null],[13,"TooManyFDsReceived","","Expected one FD, got more",1,null],[4,"Error","","",null,null],[13,"Bad","","A real error that prevents the Ring buffer from working",2,null],[13,"Limit","","An error that indicates some limit being reached. This is\nsometimes expected and realistic!",2,null],[13,"Protocol","","A protocol error (e.g., messages on the socket didn&#39;t have the\nright format)",2,null],[4,"StashableThing","","StashableThing enumerates all the things that can go &quot;into&quot; a\n[`Ring`](struct.Ring.html) buffer. Most methods / functions will\nspecify an `Into&lt;StashableThing&gt;` type.",null,null],[13,"One","","",3,null],[13,"Pair","","",3,null],[4,"StashedThing","","StashedThing enumerates all things that can come of of a\n[`Ring`](struct.Ring.html) buffer (say, when iterating).",null,null],[13,"One","","indicates that the current entry is a single file descriptor",4,null],[13,"Pair","","indicates that the current entry is another ring buffer\n(compatibility note: Stashing ring buffers does not work on\nBSD-alikes like OS X).",4,null],[5,"new","","",null,{"inputs":[],"output":{"name":"result"}}],[6,"Result","","A specialized Result type for fd Ring buffer operations.",null,null],[11,"clone","","",0,{"inputs":[{"name":"ring"}],"output":{"name":"ring"}}],[11,"fmt","","",0,{"inputs":[{"name":"ring"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"drop","","",0,{"inputs":[{"name":"ring"}],"output":null}],[11,"fmt","","",1,{"inputs":[{"name":"protocolerror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"protocolerror"}],"output":{"name":"protocolerror"}}],[11,"eq","","",1,{"inputs":[{"name":"protocolerror"},{"name":"protocolerror"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"protocolerror"},{"name":"protocolerror"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",2,{"inputs":[{"name":"error"}],"output":{"name":"error"}}],[11,"from","","",2,{"inputs":[{"name":"parseinterror"}],"output":{"name":"error"}}],[11,"from","","",2,{"inputs":[{"name":"utf8error"}],"output":{"name":"error"}}],[11,"clone","","",3,{"inputs":[{"name":"stashablething"}],"output":{"name":"stashablething"}}],[11,"from","","",3,{"inputs":[{"name":"rawfd"}],"output":{"name":"stashablething"}}],[11,"from","","",3,{"inputs":[{"name":"ring"}],"output":{"name":"stashablething"}}],[11,"clone","","",4,{"inputs":[{"name":"stashedthing"}],"output":{"name":"stashedthing"}}],[11,"from","","",3,{"inputs":[{"name":"stashedthing"}],"output":{"name":"stashablething"}}],[11,"add","","Adds an FD to a Ring, updating the count of contained FDs.\nClosing the FD to free up resources is left to the caller.",0,{"inputs":[{"name":"ring"},{"name":"t"}],"output":{"name":"result"}}],[11,"pop","","Removes and returns the head of the fd ring, updating count.",0,{"inputs":[{"name":"ring"}],"output":{"name":"result"}}],[11,"iter","","Returns an iterator on the FDs contained in the ring buffer",0,{"inputs":[{"name":"ring"}],"output":{"name":"ringiter"}}],[11,"next","","",5,{"inputs":[{"name":"ringiter"}],"output":{"name":"option"}}]],"paths":[[3,"Ring"],[4,"ProtocolError"],[4,"Error"],[4,"StashableThing"],[4,"StashedThing"],[3,"RingIter"]]};
initSearch(searchIndex);
