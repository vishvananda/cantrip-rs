// Inferno Hellion
// status: ready
// Trample (This creature can deal excess combat damage to the player or planeswalker it's attacking.)
// At the beginning of each end step, if this creature attacked or blocked this turn, its owner shuffles it into their library.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.source).is_some_and(|candidate: ObjectId| (({ let mut attackers: Vec<ObjectId> = q.history().this_turn().iter().filter_map(|r: &Record| -> Option<Fired> { let event = &r.event; if let Event::Attacks { attacker, defending, known, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(candidate) == Some(id) { return Some(Fired { subject, data }); } }  None }).filter_map(|f: Fired| f.subject.object()).collect(); attackers.sort(); attackers.dedup(); attackers.len() as i64 }) >= (1i64) || q.history().this_turn().iter().any(|r| matches!(r.event, Event::Blocks { blocker, .. } if blocker == candidate)))) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::StepBegan { step } = event { let subject = Target::Player(q.state().turn.active); let data = [0u32; 2]; if matches!(*step, Step::End) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.put_into_library(&objects_0, LibraryPlace::ShuffledIn);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Inferno Hellion", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Hellion]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(7), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::STEP_BEGAN.0), ability_1_event), ability_1, "At the beginning of each end step, if this creature attacked or blocked this turn, its owner shuffles it into their library.").targets(&[]).condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
