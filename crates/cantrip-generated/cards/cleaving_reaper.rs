// Cleaving Reaper
// status: ready
// Flying, trample
// Pay 3 life: Return this card from your graveyard to your hand. Activate only if you had an Angel or Berserker enter the battlefield under your control this turn.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.return_to_hand(&objects_0);
g.end_sentence();
 Ok(()) }
fn ability_2_if(q: &dyn Query, asker: &Asker) -> bool { (q.history().entered().filter(|e| ((e.chars(q).has_subtype(q.words(asker.source).subtype(Subtype::Angel)) || e.chars(q).has_subtype(q.words(asker.source).subtype(Subtype::Berserker))) && Some(asker.controller) == Some(e.controller))).count() as i64) >= (1i64) }

pub static CARD: CardDef = CardDef { name: "Cleaving Reaper", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Angel,Subtype::Berserker]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(life(3)), ability_2, "Pay 3 life: Return this card from your graveyard to your hand. Activate only if you had an Angel or Berserker enter the battlefield under your control this turn.").targets(&[]).condition(ability_2_if).from(ZoneKind::Graveyard)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
