// Galea, Kindler of Hope
// status: planned
// requires: operation ChangeSpell
// requires: static play permission with an effect on what is played
// Vigilance
// You may look at the top card of your library any time.
// You may cast Aura and Equipment spells from the top of your library. When you cast an Equipment spell this way, it gains "When this Equipment enters, attach it to target creature you control."
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("static-play-permission-with-an-effect-on-what-is-played" => statics: player_rule(ability_2_0_applies, PlayerRule::Play { zone: ZoneKind::Library, cards: CardsOfRule::Asked(|q: &dyn Query, asker: &Asker, player: PlayerId, card: ObjectId| (((((Some(player).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Aura)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)))).collect::<Vec<ObjectId>>()).contains(&card)), grant: PlayGrant { rider: Some(&[|q: &dyn Query, asker: &Asker, id: ObjectId| q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Equipment)), |g: &mut dyn Game, asker: &Asker, spell: ObjectId| -> Result<(), Suspend> { planned!("operation-change-spell" => if let Some(spell) = { let q: &dyn Query = g; Some(spell) } { cantrip_mechanics::spell_changes::change(g, asker, spell, &[SpellChange::Gains("Triggered")]); });
g.end_sentence();
 Ok(()) }]), ..PlayGrant::CAST } }));

pub static CARD: CardDef = CardDef { name: "Galea, Kindler of Hope", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [1, 1, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elf,Subtype::Knight]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Vigilance]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, play_rules: &[PlayRule::LookAtLibraryTop], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
