// Rivaz of the Claw
// status: planned
// requires: operation ChangeSpell
// Menace
// {T}: Add two mana in any combination of colors. Spend this mana only to cast Dragon creature spells.
// Once during each of your turns, you may cast a Dragon creature spell from your graveyard.
// Whenever you cast a Dragon creature spell from your graveyard, it gains "When this creature dies, exile it."
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (2i64).max(0) as u8 };
for player in mana_players_0 {
if mana_0 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), mana_0)?;
for color in colors { g.add_restricted_mana(player, color.mana(), 1, ManaRestriction::Subtypes(SubtypeSpells { creature: true, ..SubtypeSpells::of(Subtype::Dragon) })); }
}
}
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && (q.chars(card).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && q.chars(card).is(CardType::Creature)) && q.state().turn.active == player && !cantrip_mechanics::history::permission_used_this_turn(q, asker.source)).then(|| Cost::mana(q.card(q.chars(card).card).cost)) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && ((q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Dragon)) && q.chars(id).is(CardType::Creature)) && cast_at(q, event, id).is_some_and(|cast| cast.from == ZoneKind::Graveyard && Some(asker.controller).into_iter().collect::<Vec<PlayerId>>().contains(&q.state().obj(cast.spell).owner))) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; item.subject.object() } { cantrip_mechanics::spell_changes::change(g, asker, spell, &[SpellChange::Gains("Triggered")]); });
g.end_sentence();
 Ok(()) }
fn cast_of<'a>(q: &'a dyn Query, id: ObjectId) -> Option<&'a Cast> { let records = q.history().all(); let spell = records.iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if new == id => Some(old), _ => None }).unwrap_or(id); records.iter().rev().find_map(|r| match &r.event { Event::Cast(cast) if cast.spell == spell => Some(cast), _ => None }) }
#[allow(dead_code)] fn cast_at<'a>(q: &'a dyn Query, event: &Event, id: ObjectId) -> Option<&'a Cast> { match event { Event::ZoneChange { old, new, reason: ZoneReason::Resolve, .. } if *new == id => cast_of(q, *old), _ => cast_of(q, id) } }

pub static CARD: CardDef = CardDef { name: "Rivaz of the Claw", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Lizard,Subtype::Warlock]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).colors(ColorSet(31)).yields_any(2).only_for(ManaRestriction::Subtypes(SubtypeSpells { creature: true, ..SubtypeSpells::of(Subtype::Dragon) })), "{T}: Add two mana in any combination of colors. Spend this mana only to cast Dragon creature spells."),triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_3_event), ability_3, "Whenever you cast a Dragon creature spell from your graveyard, it gains \"When this creature dies, exile it.\"").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, granted_ways: &[GrantedWay { text: "Once during each of your turns, you may cast a Dragon creature spell from your graveyard.", permission: true, lands: false, cost: ability_2_0_cost, ..GrantedWay::CAST }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
