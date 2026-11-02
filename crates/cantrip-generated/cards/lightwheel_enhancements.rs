// Lightwheel Enhancements
// status: planned
// requires: engine: a card's own terms to cast it from its graveyard
// Enchant creature or Vehicle
// Start your engines! (If you have no speed, it starts at 1. It increases once on each of your turns when an opponent loses life. Max speed is 4.)
// Enchanted permanent gets +1/+1 and has vigilance.
// Max speed — You may cast this card from your graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Creature) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Vehicle))))) }
#[allow(unused_variables)] fn ability_2_apply_1(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Vigilance); }
#[allow(unused_variables)] fn ability_2_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((1i64) as i16, (1i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn ability_3_if(q: &dyn Query, asker: &Asker) -> bool { Some(asker.controller).is_some_and(|player: PlayerId| q.state().player(player).speed == Some(4)) }
planned_item!("engine-a-card-s-own-terms-to-cast-it-from-its-graveyard" => ways: way(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Graveyard, "Cast from your graveyard").only_from().condition(ability_3_if));

pub static CARD: CardDef = CardDef { name: "Lightwheel Enhancements", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_2_apply_1, "Enchanted permanent gets +1/+1 and has vigilance."),static_ability(Layer::PtModify, cantrip_mechanics::attach::attached_object, ability_2_apply_0, "Enchanted permanent gets +1/+1 and has vigilance.").continues()], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), start_your_engines: true, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
