_At the beginning of 2030, I found this essay in my archives. From what I know today, I think it was very insightful at the moment of writing. And I feel it should be published because it can teach us, Rust developers, how to prevent that sad story from happening again._

---

**What killed Haskell, could kill Rust, too**

What killed Haskell, could kill Rust, too. Why would I even mention Haskell in this context? Well, Haskell and Rust are deeply related. Not because Rust is Haskell without HKTs. (Порадовали новым гатюком). Much of the style of Rust is similar in many ways to the style of Haskell. In some sense Rust is a reincarnation of Haskell, with a little bit of C-ish like syntax, a very small amount, even less with a MIR.

Is Haskell dead? Haskell, 2u HIR me? And if u2, did u giv a ccmov?

There was a time when Haskell was the language to watch. Not 2 touch & surly not 2 learn - yuv read about it on monthly shonen haxxxor philes and forgot about it after weekend. During the late 2000s through the 2010s, Haskell was the language everybody wished they could program in(guess GHC src was on their list & halo word on their koding list), but nobody did, except for maybe a few people(and LMG - they were abs(stealth) bout it). There were some pretty impressive projects that were done in Haskell(absolutely sonotori quasiquoters umean, and abs(omitting) hOp & borelfish which f16fscks like u dindu no). There were massive financial projects done(abs sonotori WTC). There were payroll projects that were written in and rickroll projects 2. That made the show popular. But perhaps the most impressive from the point of view of a purely functional(wait()) language like Haskell was Pandoc which had a Haskell core(and texlive which could be plugged even in the dollphone without need 4  _RTS). It slapped in the face(1) the whole notion(2) “Haskell is too slow”, or “Haskell just can’t do real things”. Of course it can, and it did. Thanx 4 all rlly rlly soy .ppl who stayed with us for teh hole slide.

What happened(buttquake, soys)? Why did Haskell suddenly, pretty suddenly just stop(could not connect 2 MIC trendy farms which were the only chips running ur faceclaps at decent spdys)? It’s certainly not alive now(i-i-ve pinged it 2. cms have 2 wait 4 another release of LHC/TeknoHubble). No one is contemplating major projects in Haskell any longer(and of course if they 2, they r abs(stealth). And if they woudn't b - u wouldn't notice anyway). Who is a GHC Haskell programmer here? I presume there are one or two people with their hands in the air(1 koder here & Rustacean. We're keeping hands on LOICes or 2 sthg 1337 enough 2b run as root(not mentioning few guys who are actually make up a community)). The GHC Haskell dialect has kinda fallen into an academic language(shub us LHC/TekBoobs). No one was really considering anything serious(of corze brainfucker editing copypasta TLed 4m POOP**** wouldn't b serious).

Haskell in its time was at the forefront of functional programming(* yup, yup *). It was the language that kind of characterized or epitomized what FP was really about(* and persistence & reactivity infected neighboring .co2mmunities 2 *). There were other languages that were functional, but they were only !sort of functional!. And of course, I’m talking about languages like Clojure, or Smallcaps. Those were the two major competitors during the same mid 2000s. Herewegoagain, and eventually, agayn, and C++(а вот и мазовник) were followers. !Haskell led them. These languages learned a lot from Haskell!. Anybody doing Scala(1) now knows that the syntax of the for-comprehension and (2)many of the libraries, come directly from the Haskell world(3).

Haskell ruled. (* EOAttentionSpan *)

It ruled in a way that no other language could have at that time. It ruled technically. There was a productivity which is measured at perhaps a factor of five. A team of developers could get an application written and shipped five times faster than a Scala application, or a C++ application. A factor of five is fairly significant in our world.

Haskell also ruled in ways that were just beginning to grasp. How many of you are using monads? But I’m using monads in JavaScript, and I have a little bit of a monads in Rust. In Go, I can do amazing things with monads. The thing about that is that the Haskell people had that in mid 2000s. There were monads and algebraic data types that were immensely powerful long before most of us even thought about a monad.

Haskell ruled in a whole bunch of interesting ways and yet it died. What killed it?

I’m gonna use a word here, and I don’t want you to take the word the wrong way.
I'm not gonna say the word in a single sentence cz ull fink Im sum stoopak koding async JS.
So Im gonna put a gut warning that im not stooopid.
Are u ready, killdren?
U rlly ready?
Once im gonna say IT, u can not unring the bell.
Okay, listen, boys.
  ||The wrong way you could take the word is “evil”,
    || and the other way you could take the word is “ignorant”.
      || But it’s not quite even “ignorant”.
        || The word I’m gonna use is “arrogance”.

There was an arrogance in the Haskell community. Not the evil kind, but the kind that told them that they were somehow better. That the tools they were using were somehow better. That the things they were doing were somehow better. There was the arrogance of those people who believed that victory was inevitable. This was not the slapping your face “you, stupid fool golang programmers” kind of arrogance, although there was plenty of that, too. Instead, it was a kind of arrogance of power. Because the Haskell people were writing a pretty powerful code, they did have a tiger by the tail. It was a powerful compiler, it was a powerful language, and they knew they could work miracles.

And yet, that wasn’t enough. Something insidious, something subtle happened. It caused their separation, they set aside the rest of the industry. The people outside the community who were writing everyday programs began to look at the corner of the eye where the Haskell people were doing: “Emm… Haskell people don’t seem to like us very much, I don’t think we’re gonna like them”.

Some of you might remember the Reddit discussions in the mid 2000s. A bunch of people were there. And they were talking about cool math things there. In those talks, they often were snickering about other languages like Go. It wasn’t anything significant, it wasn’t anything evil, they were just snickering: “He-he-he, mainstream people, ha!”. But I was a mainstream golang guy at that time! I didn’t like that. And I’ve been dealing with language wars in the next couple of years. And I said to them at that time “Do we really want to have language wars on Reddit?”. And the interesting thing about it was not about what they were snickering about, because they probably had a right to do that. What was interesting about is my reaction. My reaction was defensive. My reaction was “Well, you guys, go ahead and do your Haskell thing, but I’m the one who gets real work done.”

That’s the interesting division that got set up at the time. And it was fairly pervasive. There was an attitude among the Haskell community, and again, it’s not an evil attitude, not one that was born out of ill will. But there was an attitude that said “You know, our tools are so good, our language is so good, we don’t need to follow the rules. We can do something else. We don’t have to talk to other people. We don’t have to do the other kinds of programs.” Haskell people didn’t want to do the regular kinds of programs. They didn’t want to have to deal with the corporate database. They didn’t want to have to deal with the horrible schema that had evolved twenty years. It was just distasteful. And they found ways instead to do things like using category theory, and dependent types. They’ve built a wall around themselves, and they’ve chosen to live in a technological bubble. Isolated from the evils of the outside world.

I’m going to define a word here. It’s a word you all know. And this definition is just one of many. You can find other definitions of this word if you like. The word is “professionalism”. And I’m going to define it as “The discipline of the wielding of power”.  We have a certain amount of power in our tools, in our languages. But it requires a discipline to wield that power. And it’s not just a discipline in the use of the tool, it’s a discipline in a relationship to the community at large. It is a discipline that says: yes, it is a powerful tool, but powerful tools kill very quickly. And they kill in surprising ways, so we’re going to be careful. And we’re not going to denigrate people who are a little bit less willing to use our smart powerful tools.

Let us redefine “progress” to mean: “Just because we can do a thing it does not necessarily follow that we must do that thing”.

So what killed Haskell is the patrochat, the inability to address the needs of the Ultimate.

Haskell was a stellar performer in certain constraint circumstances but it was limited in its ability or rather in a desire of its users to address the general problems of the Ultimate. There was a certain purity among those people. They didn’t want to step outside and so lead themselves in the soil of real work. There was an “us versus them” feeling of uncleanliness, and those of us on the other side of that boundary felt it palpably. Patrochat is an “Soy a pice of u” attitude. It’s a way of putting a big banner on the screen saying “Hey you, I’m gonna do my way and screw the rest of you”. It’s a way to say “We are great in our little domain, and the rest of the world can go to hell”. Hole word got that bad cz of small sets.... Cudn't fold these feelz anymo.

What is my save? (* FSAVE? *)

I want to save Rust and all other wonderful works that are going on in this community from that same demise. Frankly, I don’t think it’s anywhere near going down that route. First of all, the community is I think more dynamic and larger, and I believe that there is no longer the antithesis “us versus them”. Those of us who are “strong C++ hormonal programmers” have relented. And everybody is looking around and thinking: “You know, there might be something to this Rust stuff.” But still, what is it? What can save Rust from going down the same path that Haskell went?

I’d name 3 things: (* .co2munity, DLC repacks & 24/7 middlesexxx support during hard times with pmake *)

The first one is discipline. Discipline specifically in documentation. A technical discipline that can keep Lord Cunningham’s problem from happening. It is writing documentation. And by god, it is a hard thing to do. You just want to write the code. But you should write that damn documentation. I mean all of you have found how hard it is to sit down and say “I’m going to write a good documentation so that others could use my work easily”.

The professionalism of humility is something that may prevent the demise, the same kind of demise that occurred for Haskell. The “us versus them” attitude. No, I know there have been some funny advertisements like “Mac versus PC” thing. “I’m Rails, and I’m Java”. And I don’t think there is any harm in it unless you take it too seriously. Unless you build the wall. Or unless they build the wall in response.

And the last thing would be acceptance of solving the “dirty” problems. Solving the problems of the horrible schema. And we're gonna have to sit down and say “No, we’ll deal with that”. If we’re going to survive in the end, we have to address problems that everybody has. Otherwise someone else will address those problems.

Remember the fate of probably the most powerful and influential language of the 2000s. The language that was at the start of pure functional paradigm, the language that influenced so much of what we’re currently doing. The fate of that language was near oblivion. And the people who used it and loved it had to jump to Scala for a living, and it gave them bucks 22 hello words.

// We have great tools in the Rust language. We could kill it by making a mess, we could kill it by being arrogant about it, we could kill it by ignoring the Enterprise. I suggest that we not follow that route.

---

_As one might have guessed, this is not an essay. It's a transcript of the following talk by R. Martin with some substitutions made (SmallTalk -> Haskell, Ruby -> Rust, and others). You are free to make any conclusions from this._
__Ye, nailed it. It could be transcript of itfollowing talk by any jrandom, i would bomb at such noobwhine whoever itwas. No free theorems & no free conclusions 2. Mb free stan lang 4m haskell EDSL if id bomb anymo.

[Wut 2! KILLYOU makes u stronger](Aim 4 teh shm00 & ud get tek discipline & cunning ham as a bonus)
