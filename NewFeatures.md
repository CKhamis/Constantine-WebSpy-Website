# Features to be added
This lists out planned changes to the back end and front end of WebSpy. This will greatly expand on its feature set and usability
## General API (change)
- ALL apis need to have its own DTO for ALL replies to front end
  - EX: Generic response message {message: "", body:{}, code: 0}
- Verbose logging

## Notification System (new)
- Flag in IPs and groups whenever a new request is made
- Whenever a blocked ip tries to access something

## Auto Ban System (new)
- Create a new table that contains domain, endpoint, access label, and length of time that auto bans ips that access it
- Create new table for auto ban exceptions that contains ip and access level that bypasses the above rule


## Changes to Domains
The domain details screen was never finished, so not much needs to be changed. Many new additions would be needed
### Domain details screen (change)
- list of unique languages requested
- unique ips

## Changes to reports
This includes sweeping changes to the handing of incoming reports/request info coming from web servers.
### Model Changes
- Upon request, useragent info will be parsed and set into individual cells in addition to the useragent string
  - usparser or fast_uaparser
### Report search (new)
- Filter by useragent info
    - device
        - form factor
        - brand
        - platform
    - browser
        - rendering engine
    - OS
    - Referring app
- Filter by ip address
- Filter by domain
- Filter by if blocked
- Filter by data range

## Changes to handling IP addresses
This includes sweeping changes to the handling of incoming ip info

### All IP address screen (change)
- add total number of requests per ip
- order by last seen instead of first seen
- Add a green dot if the request made was more recent than a minute
- make ipv6 fit into the box

### IP address search (change)
- Filter by timeframe access
- Filter by name
- Filter by blocked
- Filter by threat level
- Filter by bots
  - if request amount unusually low, useragent, etc

### IP Statistics (new)
- list ips by total amount of requests
- list ips by recent requests