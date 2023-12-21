# tailchat

tailchat is a minimal/non scalable/crazy/possibly fun take on p2p chatting.

This is an experiment mostly for fun and to explore the possibilities when you
have a secure small network and want to chat with people inside it.

Not a novel idea... We know... but we also want to play around with some Rust :-).

The major architectural constraints are:

1 - Communication is always peer to peer
2 - Any storage (if any) is stored only on the peers participating on the conversation
3 - The network is secure (protocols will be plain text/binary, no TLS)
4 - Peer to peer communication is always possible

The constraints and the overall design are inspired on the ideas of [Remembering the LAN](https://crawshaw.io/blog/remembering-the-lan),
a time where building networked software was easy and simple (we are that old):

```
The LAN was a magical place to learn about computers.
Besides the physical aspect of assembling and disassembling machines,
I could safely do things unthinkable on the modern internet: permission-less file sharing,
experimental servers with no security, shared software where any one machine could easily bring down
the network by typing in an innocuous command.
Even when I did bring down the network the impact never left the building.
I knew who I had to apologise to.
```

Technology like wireguard/tailscale helps us go back to those simpler times by building "LANs" on top
of the Internet. We want to explore building things on top of this model, where application code
can just trust other peers and doesn't need to deal with the hassle of making peer to peer communication
work (NAT traversal is solved by the network layer, or NAT is not an issue for you like in a LAN).

In general the project should work fine in a mesh VPN like Tailscale, but would also work fine in an actual LAN.

## Installing

TODO

## Basic usage

TODO

## User Discovery

TODO
At some point document different methods of user discovery ? For now start with adding IP/Hosts manually and all good :-).
