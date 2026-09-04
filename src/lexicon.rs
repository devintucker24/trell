use std::collections::HashMap;
use std::sync::OnceLock;

pub const FEATURES: &[&str] = &[
    "heat", "cold", "near", "far", "soft", "hard", "light", "dark", "wet", "dry", "body",
    "machine", "kin", "stranger", "speech", "silence", "brief", "lasting", "sure", "doubt", "care",
    "neglect", "life", "death", "small", "large", "inside", "outside", "order", "chaos", "past",
    "future", "sacred", "profane", "play", "work", "animal", "plant", "war", "peace", "child",
    "elder", "food", "sleep", "home", "road", "money", "law", "night", "day", "medical", "paper",
    "emotion", "reason", "violent", "gentle", "formal", "casual", "urgent", "wait", "water",
    "fire", "empty", "full",
];

const RAW: &str = r#"
abandon neglect,far,past,empty,emotion
absence empty,far,neglect,silence
always sure,lasting,future
am perhaps doubt,speech
among near,large
and wait
anger emotion,heat,violent,hard
anxious doubt,urgent,emotion,body
apart far,neglect,empty
arm body,near
ashes death,past,cold,empty,fire
ask speech,doubt,near
asleep sleep,night,soft,silence
authority law,formal,hard,sure,elder
awake day,light,body
baby child,soft,small,kin,life
bare empty,cold,body
bed sleep,home,soft,inside,small
beside near,kin
betray neglect,far,emotion,violent
between near,doubt
birth life,child,wet,future
blood body,life,wet,violent,heat
blotter paper,law,order,cold,formal
body body,near,soft
bone body,hard,death,dry
book paper,reason,lasting
boy child,kin
breath body,life,air,soft,brief
brief brief,small
bright light,heat,day
brother kin,near,male
building large,outside,hard,machine
burn fire,heat,violent
calm peace,soft,wait,silence
care care,near,kin,gentle,emotion
caress soft,body,near,gentle,heat
casual casual,speech,near
century lasting,large,past,future
chart paper,medical,order,reason
child child,small,kin,life,soft
church sacred,inside,silence
city large,outside,machine,noise
clean order,dry,light
clinical cold,medical,hard,reason,order
close near,soft,inside,kin
closed order,hard,inside,empty
cloth soft,body,near
cold cold,far,hard,silence,death
color light,emotion
comfort care,soft,home,heat,near
command speech,sure,law,hard
corridor inside,far,empty,medical
country large,outside,home
courage sure,heat,life,war
cradle child,soft,home,sleep
crime law,violent,profane
cry emotion,speech,wet,body,child
dad kin,elder,male
damage violent,hard,chaos
danger war,urgent,violent,doubt
dark dark,night,silence,far
darling kin,care,emotion,near,soft,heat
daughter kin,child
dawn day,light,future,brief
day day,light,work
dead death,cold,empty,past,silence
death death,cold,empty,past,dark
dear kin,care,emotion,near,speech
definite sure,hard,reason
desk paper,work,hard,order
die death,body,past
dinner food,home,kin,night
distance far,road,cold
distant far,cold,silence,stranger
doctor medical,care,reason,elder
door home,inside,hard
doubt doubt,speech,wait
down far,dark
dream sleep,night,emotion,chaos,inside
dry dry,hard,cold
each large,order
eager urgent,heat,future
earth plant,outside,home,large
east outside,road
easy casual,soft,wait
edge far,hard,brief
ember heat,fire,soft,near,small,home
emotion emotion,body,wet
empty empty,far,neglect,silence,cold
end death,past,brief
enough full,sure
escape road,far,urgent,fear
even wait
ever lasting,sure
every large,order
everyone large,stranger,speech
everything large,full
evidence paper,reason,law,sure
exact sure,order,reason,hard
except far
face body,near,speech,emotion
fact reason,sure,paper
fade empty,past,soft,wait
fail neglect,empty,past
family kin,home,near,care
father kin,elder
fear emotion,cold,dark,doubt,body
feel emotion,body,soft,near
felt emotion,body,past
field plant,outside,large
figure paper,reason,large,order
figures paper,reason,large,order
find reason,sure
fire fire,heat,violent,light
first brief,order
fluorescent light,machine,cold,medical
folk kin,home,casual
follow road,near
food food,life,home
for wait
force hard,violent,sure
formal formal,speech,law,paper,order
found past,sure
friend kin,near,care,speech
from far
full full,near,life
gentle gentle,soft,care,slow
girl child,kin
give care,near
glad emotion,heat,play,light
glass hard,cold,light,empty
go road,future,brief
god sacred,large,sure
gone empty,past,far,death
good care,gentle,emotion
government law,large,formal,paper,order
grave death,earth,past,sacred,silence
great large,sure
green plant,life,outside
grief emotion,death,wet,kin,empty
ground earth,hard,outside
grow plant,life,future,child
guard law,hard,war
guide care,road,speech
hand body,near,care,work
hands body,near,care,work,soft
hard hard,work,sure
hate emotion,violent,far,heat
he body,stranger
head body,reason
heal care,medical,life,soft
health life,body,medical
hear speech,body,near
heart body,emotion,kin,life,heat,near
heat heat,fire,near,urgent
held near,care,soft,kin,body,gentle
hello speech,casual,near
help care,near,urgent
her kin,body,near
here near,home,brief
herself body,inside
high large,outside
him body,stranger
his body
hold near,care,body,sure
home home,inside,kin,care,heat,soft
hope future,emotion,light,doubt
hospital medical,inside,machine,care,order
hot heat,fire,urgent
hour brief,time,wait
house home,inside,hard
how doubt,speech
human body,kin,life
hurt body,violent,emotion,pain
i emotion,speech,body,near
ice cold,hard,dry,far,silence
if doubt,future
ill medical,body,death,cold
immediate urgent,brief,sure
insist sure,speech,hard,urgent
instrument machine,medical,hard,cold
instruments machine,medical,hard,cold,order
intimate near,soft,kin,emotion,heat,inside,body
into inside,near
is sure
it stranger,far
its stranger
join near,kin,full
joke play,speech,casual,heat
judge law,reason,formal,sure
just law,reason
keep care,near,lasting,sure
kept care,near,past
kin kin,near,home
kind care,gentle,emotion
kitchen home,food,inside,small,heat
know reason,sure,inside
known sure,past
lack empty,neglect
lady elder,formal
laid order,body,past
lamp light,home,night,small
land earth,large,home
language speech,reason
large large,full
last lasting,past
late night,wait,past
law law,order,formal,hard,paper
lay body,sleep,soft
lead road,sure,elder
leave far,road,neglect,past
left far,past,empty
letter paper,speech,near,emotion,home
lie speech,doubt,profane
life life,body,heat,future
light light,day,heat,soft
lights light,machine,night,cold,medical,order
like near,emotion
line order,paper,hard
listen speech,silence,care,near
little small,child,soft
live life,home,body
long lasting,far
look light,body,near
lose neglect,empty,far,past
loss empty,death,emotion,past
lost far,empty,dark,past
love emotion,kin,heat,near,soft,care,life
low small,dark
machine machine,hard,work,cold
made work,past
man body,elder
many large,full
mark paper,order
may doubt,future
maybe doubt,speech,wait
me body,near,emotion
mean reason,speech
measure order,reason,paper,sure
medical medical,body,care,reason
memory past,inside,emotion,lasting
men body,large
might doubt,future
mind reason,inside,speech
minute brief,wait,urgent
miss neglect,far,emotion,empty
mom kin,elder,care
money money,work,hard
month lasting,wait
more full,large
morning day,light,home
mother kin,care,home,elder
move road,body,brief
mr formal,speech,stranger
must sure,urgent,law,speech
my kin,near,home
name speech,sure,near
nation large,law,outside,order
near near,home,soft
need urgent,empty,care,body
neighbor near,home,stranger,kin
neither far
never sure,lasting,far
new future,light,child
next future,near,brief
night night,dark,sleep,silence,cold
no sure,hard,far
none empty,far
nor far
north far,outside,cold
not far,hard
note paper,speech,small
nothing empty,far,silence,death
now urgent,brief,near,sure
number paper,reason,order
nurse medical,care,body,work,night,order,machine
oath sure,sacred,speech,law,lasting
of wait
off far,dark
office work,paper,formal,inside,order
often lasting
old elder,past,lasting,home
on near
once brief,past
one small,near
only small,sure
onto near
open outside,light,near
or doubt
order order,law,reason,hard
other stranger,far
our kin,near,home
out outside,far
over large,above
own home,near,sure
pain body,violent,emotion,hard
paper paper,order,reason,dry
parents kin,elder,home,care
part small,near
party play,speech,heat,kin
pass road,brief,past
past past,lasting,memory
path road,outside,near
peace peace,soft,silence,home
people large,stranger,speech,body
perhaps doubt,speech,wait,soft
person body,stranger
phone speech,machine,near,urgent
place home,earth,near
plain casual,dry,reason
plan future,reason,order,paper
plant plant,life,outside,green
play play,child,casual,heat
please speech,care,soft,near
poem speech,emotion,paper,sacred
point reason,small,hard
police law,hard,order,violent
poor money,empty,neglect
power hard,large,sure,law
present near,gift,brief
press paper,speech,large,formal
pretty light,soft,emotion
probably doubt,reason
problem chaos,hard,work
public large,outside,speech,formal
pull body,near,hard
put order,near
quiet silence,soft,night,home,peace
quite sure,small
radio speech,machine,far
rain water,wet,cold,outside
rather doubt,speech
read paper,reason,speech
ready sure,future,urgent
real sure,reason,hard
really sure,emotion
reason reason,speech,order,cold
red heat,fire,blood,emotion
region large,outside,earth
release paper,law,speech,future
remain lasting,near,wait
remember past,inside,emotion,kin
report paper,reason,formal,speech,order
rest sleep,peace,body,wait
return road,home,future,near
rich money,full,large
right sure,law,reason
rise light,future,body
river water,wet,outside,lasting
road road,outside,far,machine
room inside,home,small
run road,body,urgent
sad emotion,wet,empty,cold,past
safe home,care,peace,sure
said speech,past
same order,sure
save care,future,urgent
say speech,near
scale large,reason,order,measure
scene outside,light,emotion
school child,work,inside,reason
sea water,large,far,wet
second brief
secret inside,silence,dark,near
hidden inside,silence,dark,far,secret
whisper speech,silence,near,inside,soft
see light,body,near
seem doubt,speech,soft
seems doubt,speech,soft
self body,inside,reason
send paper,road,speech,far
sent paper,past,far
set order
several large
shadow dark,far,past,silence,empty
shall future,sure
she body,kin
shift work,night,body,medical
shine light,heat,day
ship water,road,large,machine
should doubt,speech,law
show light,speech,near
sick medical,body,cold,empty
side near
sign paper,speech,order
silence silence,night,soft,far,cold
silent silence,night,soft
simple small,casual,soft
since past
sing speech,emotion,play,music
sister kin,near,care
sit body,wait,home
six number
sleep sleep,night,soft,body,silence,home
slow wait,soft,lasting
small small,child,soft,inside
snow cold,wet,white,silence,outside
so speech
soft soft,gentle,body,heat,care
soldier war,hard,body,death
some small,doubt
someone stranger,near,body
something stranger,doubt
sometimes doubt,wait
son kin,child
song speech,emotion,play
soon future,brief,urgent
sorry emotion,speech,care,past
soul sacred,inside,emotion,lasting
sound speech,near
south outside,heat
speak speech,near,reason
speech speech,mouth,reason
stand body,sure,hard
star light,night,far,sacred
start future,brief
state law,large,formal,paper
stay home,near,wait,lasting
still silence,wait,lasting
sterile cold,medical,hard,order,machine,dry
stone hard,cold,earth,lasting
stop hard,brief,sure
story speech,past,paper,emotion
street road,outside,city
strong hard,body,sure,life
study reason,paper,work
such large
sudden urgent,brief,chaos
suffer pain,emotion,body,lasting
summer heat,day,outside,life
sun light,heat,day,outside,life
sure sure,hard,reason,speech
swear sure,speech,sacred,heat,oath
sweet food,soft,care,heat,child
system order,machine,large,reason
table home,food,hard,inside
take near,body
talk speech,casual,near
tea food,home,heat,soft,wait
tell speech,near,sure
tender soft,care,kin,emotion,body,gentle,heat
than reason
thank speech,care,near
that stranger,far
the wait
their stranger,far
them stranger,far
then past,future
there far,outside
these near
they stranger,large
thing stranger,small
think reason,inside,doubt,speech
thinking reason,inside,doubt,emotion
this near,brief
those far
though doubt
thought reason,past,inside
three small
through inside,road
time lasting,wait,reason
to road
today day,near,brief
together near,kin,full,home
told speech,past
tomorrow future,day
too full
took past,body
toward near,future,road
town small,home,outside
tree plant,outside,lasting,life
true sure,reason,sacred
trust care,sure,near,kin
try work,doubt,future
turn body,brief,road
turned body,past,brief
two small,near
under far,dark
until wait,future,lasting
up light,future
upon near
us kin,near
use work,machine
very sure,full
village home,small,kin,outside
voice speech,body,near,emotion
wait wait,silence,future,soft
walk road,body,outside,brief
wall hard,inside,home,far
want emotion,empty,future,near
war war,violent,death,hard,fire
ward medical,inside,order,night,empty,cold,machine
warm heat,soft,near,home,care,gentle
warmth heat,soft,near,home,care,body
was past,sure
water water,wet,life,soft
way road,reason
we kin,near,speech
week lasting,wait
well care,water,sure
went road,past
were past
west outside,road
wet wet,water,body
what doubt,speech
when time,doubt
where far,doubt
whether doubt
which doubt
while wait,lasting
white light,cold,clean,snow
who stranger,doubt,body
whole full,large
why doubt,reason,speech
wide large,outside
wife kin,home,near,care
wild chaos,animal,outside,violent
will future,sure
wind air,outside,far,cold
window light,home,inside,glass
winter cold,night,outside,death,dry
wish future,emotion,doubt,speech
with near
without far,empty,neglect
woman body,kin
women body,large
wonder doubt,speech,emotion,soft,inside
wood plant,hard,home
word speech,paper,small
words speech,paper
work work,hard,day,machine
world large,outside,earth,full
worry doubt,emotion,future,urgent
would doubt,future
write paper,speech,reason,lasting
wrong far,chaos,profane
year lasting,time
yes sure,speech,near
yet wait,doubt
you kin,near,speech,body
young child,life,future,heat
your kin,near
"#;

pub fn feature_index(name: &str) -> Option<usize> {
    FEATURES.iter().position(|feature| *feature == name)
}

pub fn lookup(word: &str) -> Option<&'static [u8]> {
    table().get(word).map(|features| features.as_slice())
}

fn table() -> &'static HashMap<String, Vec<u8>> {
    static TABLE: OnceLock<HashMap<String, Vec<u8>>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        for line in RAW.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut parts = line.splitn(2, ' ');
            let word = parts.next().unwrap();
            let Some(tags) = parts.next() else {
                continue;
            };
            let mut features = Vec::new();
            for tag in tags.split(',') {
                let tag = tag.trim();
                if tag.is_empty() {
                    continue;
                }
                if let Some(index) = feature_index(tag) {
                    features.push(index as u8);
                }
            }
            if !features.is_empty() {
                map.insert(word.to_string(), features);
            }
        }
        map
    })
}

pub fn stem(word: &str) -> String {
    let lower = word.to_ascii_lowercase();
    if lookup(&lower).is_some() {
        return lower;
    }
    for suffix in ["'s", "es", "ed", "ing", "ly", "s"] {
        if let Some(stripped) = lower.strip_suffix(suffix) {
            if stripped.len() >= 3 && lookup(stripped).is_some() {
                return stripped.to_string();
            }
        }
    }
    lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn knows_warm_and_clinical_words() {
        assert!(lookup("ember").is_some());
        assert!(lookup("ice").is_some());
        assert!(lookup("darling").is_some());
        assert!(lookup("fluorescent").is_some());
        assert_eq!(stem("nurses"), "nurse");
    }
}
