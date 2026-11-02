// Temple of the Dead // Aclazotz, Deepest Betrayal
// status: ready
// (Transforms from Aclazotz, Deepest Betrayal.)
// {T}: Add {B}.
// {2}{B}, {T}: Transform this land. Activate only if a player has one or fewer cards in hand and only as a sorcery.
// Flying, lifelink
// Whenever Aclazotz attacks, each opponent discards a card. For each opponent who can't, you draw a card.
// Whenever an opponent discards a land card, create a 1/1 black Bat creature token with flying.
// When Aclazotz dies, return it to the battlefield tapped and transformed under its owner's control.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_3: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_3: Vec<ObjectId> = Vec::new();
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5 = vec![Default::default(); players_0.len()];
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let amount_2 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_2: Vec<PlayerId> = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_2 = Vec::new(); for player in players_2 { cards_2.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_2)?); } let discarded_2 = g.discard_together(&cards_2);
let paid_1 = !discarded_2.clone().is_empty();
 each_3[index_0] = discarded_2.clone(); all_3.extend(each_3[index_0].iter().copied());
each_4[index_0] = { let q: &dyn Query = g; discarded_2.clone() }; all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = paid_1;
   } 
g.end_sentence();
let amount_6 = { let q: &dyn Query = g; (({ let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = { let mut all = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); let other = q.state().apnap().filter(|&bound_0: &PlayerId| !(Some(bound_0).and_then(|p: PlayerId| players_0.iter().position(|&next| next == p)).is_some_and(|at| (each_5[at] as i64) != 0))).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all }; all.retain(|p: &PlayerId| other.contains(p)); all }).len() as i64).max(0) as u32 }; let players_6 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_6 { g.draw(player, amount_6); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Discarded { card, player, undefined } = event { let id = *card; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&*player) && q.chars(id).is(CardType::Land) && !*undefined { return Some(Fired { subject, data }); } }
 None }
pub mod token_8 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Bat).name, token_words: token!(Bat).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bat]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_7 = Vec::new();
let count_7_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_7_0 = g.card_id(&token_8::CARD);
for player in players_7 {
tokens_7.extend(std::iter::repeat((def_7_0,player)).take(count_7_0 as usize));
} let words_7 = g.words(asker.source);
let created_7 = g.create_tokens_worded(&tokens_7, false, None, words_7)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *old; let subject = Target::Object(id); let data = [new.0, u32::MAX]; if to.is(ZoneKind::Graveyard) && from.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_9 = ({ let q: &dyn Query = g; ((Some(ObjectId(item.data[0])).filter(|o| *o != ObjectId::NONE).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&id| q.state().obj(id).became.is_none() && q.state().obj(id).zone.kind() == ZoneKind::Graveyard).collect::<Vec<ObjectId>>()).get(0).copied().into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_9 = { let mut moved: Vec<ObjectId> = Vec::new(); for &o in &objects_9 { let controller = g.state().obj(o).owner; let entered = g.put_onto_battlefield_transformed(o, controller, true)?; if g.is_on_battlefield(entered) { moved.push(entered); } } moved };
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Aclazotz, Deepest Betrayal", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Bat,Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_2_event), ability_2, "Whenever Aclazotz attacks, each opponent discards a card. For each opponent who can't, you draw a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::DISCARDED.0), ability_3_event), ability_3, "Whenever an opponent discards a land card, create a 1/1 black Bat creature token with flying.").targets(&[]),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_4_event), ability_4, "When Aclazotz dies, return it to the battlefield tapped and transformed under its owner's control.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_8::CARD];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
g.add_mana(player, Mana::B, mana_0_0);}
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { g.transform_by(object, item)?; }
g.end_sentence();
 Ok(()) }
fn ability_1_if(q: &dyn Query, asker: &Asker) -> bool { (q.state().apnap().collect::<Vec<PlayerId>>()).into_iter().any(|bound_0: PlayerId| ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) <= (1i64)) }

pub static CARD: CardDef = CardDef { name: "Temple of the Dead", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 1, 0, 0, 0]), ..add_by(ability_0_mana) }, "{T}: Add {B}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{2}{B}, {T}: Transform this land. Activate only if a player has one or fewer cards in hand and only as a sorcery.").targets(&[]).sorcery_speed().condition(ability_1_if)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
}
