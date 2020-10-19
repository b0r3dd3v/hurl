[Note]
Userscript has normal linux identation - microhub doesn't display it and M$ won't either(mah guess). Feel free 2 prettier(beautify) it or do some enters after semicolons and ur done.
Hmm, zsh/ion looks good only in Raw - just remember 2 write scraptor stuff after the shell got u on newline(zsh on windows?). On Linux everything is going 2b okay.


[INSTALL]
cargo install scraptor --git https://github.com/b0r3dd3v/scraptor
Шоб було, а то я не могу позволить устаревшему коду быть ореховей нас :P .

[Update]
Now it finally works, so it is time 2 share some sickrets about using scraptor. // of.coarse? it works. Яжепочинил.
It's CLI looks the same 2 user(as old version, is not maintained 4m looks of it), tho I have extracted clap stuff into yaml document, because 2 much shit was in the main module.
Both of the utilities can be used 2 scrape follows(getting ur follows array or you simply process your bookmarks / _ is another story).
Since they have caps 2 scrape teh whole manga(thanx, baaaaka Holo, 4 ur braindead API design), you iterate over array of your manga IDs(these, appearing in manga URL / API calls with "type=manga" stuff(were POSTs, but now just "fetches")
How did I shot Web/3.0:
 
[Prepare urself 4 tldr]

Create TamperMonkey script(dunno bout GreaseMonkey, but should be runnable 2):

================%COPYPASTA_BEGAN&================
// ==UserScript==
// @name         IdMiner
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the tty1!
// @author       admin-scum
// @match        https://mangadex.org/follows*
// @grant        none
// ==/UserScript==

(function() {
    'use strict';
   const qwery = document.body.querySelectorAll(".manga-entry");
   const urwaifu = [...qwery].map(x => x.attributes[1].nodeValue);
   console.dir(urwaifu);
})();
==============%END_OF_CPPASTA%=================

Go to ur follows page(now it is "https://mangadex.org/follows/manga/0", admin scum will change it - just look 4 simple list in View mode(chocolate button over sorting pots) with DevTools launched and in Console tab.
Enable IdMiner script on the page and refresh.
In console, Array[72] or Array[%ur_harem_strength_here] will appear - it will store all ur Follows 4m the same page(up 2 100, need 2b rinse&repeated if ur manga Starbucks and have that many series in the Follows).
Click on Eastbound triangle on the line with Array and select it as text and copy 2 file.(u can send it via debug websocket / display it as nifty HTML with br tags / _ - that is, if
ur up 2 some PiTA).
// Upd: Xi is only passively maintained. Thoughts and prayers 2 the community.
In Atom/Xi/Sublime(I reccomend Xi && xi-gtk, packaged with void; Atom is slow, but good - if u don't care about txt editors, it is sufficient) use Ctrl-F arts 2 select ': "' substring, close the search,
and uncheck 0..9 elements(Ctrl-click on the cursor 2 uncheck, same on middle of nowhere 2 create cursors).
Remove other ': "' substrings(single Backspace/Del) and numbers @beginning(u can move all cursors simultaneously) and press end. Whatever length of manga IDs were, u'll get past the second prims/quotes - remove them 2.

That's it. U have ur follows list out of Google's HTTP/3 client.
Hold on to this list, admin scum makes it harder 2 us JS and API - one day ull have 2 parse HTML or make urself a GUI. Or maybe MD will take a DMCA notice ordering it 2 commit seppuku and admins will make a normal Export interface.

--------
The actual usage(after u got number-per-line list of manga IDs:
[requires zsh/ion-shell, both are packaged in Void Linux; if ur on Arch/Gentoo/subLinuxes - install zsh]
Create a subvolume on btrfs. If u don't know wut it is, don't bother.
Create a folder 4 the 6 hour anime & manga education session - there is no resume 4 scrapes, so the most rel way is 2 pwn the folder and rescrape.

cd into it, iterate over file:

ion-shell:
for i in $(cat ../madoka.stash)
  scraptor $i
  end

zsh:
for i in $(cat ../madoka.stash)
  scraptor $i

З.Ы. It can be tmpfs, if u have enough RAM(8G 4 me) 4 dling some series b4 baconing them 2 disk(btrfs or CoW FSes are good 2 go) and scraping pruned copy of faplist(since MD servoverse 2! correctly serve their crap 4 entire night).

----------
That's how it's done. If ur keeping ur follows list of MD, ur getting out of trouble - only API changes(2b harder 2 use). I will track the changes
until they roll out DRM, JSON Crypto and JWE, when I'll make a new tool 2 scrape them even harder.
The scraper is Fast - and it supports MDNet. Normally, it wouldn't be hitting server in a distributed system, but MDNet has no DHT of sort - it has API,
because it can be obfuscated and made harder 2 use(that's intentional).
So, after u've scraped it - DDoS it's servers by something. Their IPs are visible by "!shodan mangadex.org" search in DuckDuckGo(not anymo, but keep watching their IPs) - copy the ones near MD logo
and whois / GeoIP them, if u wanna drive 2 teh gateway(not 1337); sthg L4 - 2 do real damage (there is mound foory over r/mangadex or Discord or whenever
these pindos hang up during SHPStorm; not anymoo) without any legal consequences.

And yah, forgot 2 say - subvolume & btrfs are good because lots of unsuccesful scrape retries will leave soft deleted files on the volume,
which can slow down the system.
But on btrfs (btrfs subvolume create /path/2/subvolume) or tmpfs (the /tmp one) it doesn't even matter.
// As of now, MDNet nodes stimes cunt keep up with dling entire Atsumare, so u can dl 2 /tmp and copy succesful series 2 fap folder, when u dled it suc.

[Old Readme 4m //github.com/dyedquartz/mangadex-scraper]
Usage
Download latest version from github or cargo and run mangadex-scraper <ID> // 1337 is teh way 2go. Run as scraptor -l $MOONSPEAK $ID . Or iterate over aqua-per-line file with zsh/ion.

Manga ID is /title/22723/sewayaki-kitsune-no-senko-san

Chapter ID is /chapter/8857/1

use mangadex-scraper --help for more info and to list options

can also be installed through cargo by running cargo install mangadex-scraper // It's me again, iDindu pub it 2 crates.io yet. Ye can fork it and cargo pub 4m ur own username, tho.

Contributing
on the off chance anybody wants to contribute, clone the repository and run cargo run. // Contributing here is mo about publishing ur archive.weeb 2 torrents after MD is done.

[Contributing]
Make a gist and gimme link. Or make a PR(i dunno bout receiving PRs neither can make them... .dat baka).
Srsly, it seems 2b a perfect 2l.
