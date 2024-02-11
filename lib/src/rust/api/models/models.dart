// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.22.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class Artist {
  final String artist;
  final String url;

  const Artist({
    required this.artist,
    required this.url,
  });

  @override
  int get hashCode => artist.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Artist &&
          runtimeType == other.runtimeType &&
          artist == other.artist &&
          url == other.url;
}

class Character {
  final String character;
  final String url;

  const Character({
    required this.character,
    required this.url,
  });

  @override
  int get hashCode => character.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Character &&
          runtimeType == other.runtimeType &&
          character == other.character &&
          url == other.url;
}

class GalleryFiles {
  final int width;
  final String hash;
  final int haswebp;
  final String name;
  final int height;
  final int hasavif;
  final int? hasavifsmalltn;

  const GalleryFiles({
    required this.width,
    required this.hash,
    required this.haswebp,
    required this.name,
    required this.height,
    required this.hasavif,
    this.hasavifsmalltn,
  });

  @override
  int get hashCode =>
      width.hashCode ^
      hash.hashCode ^
      haswebp.hashCode ^
      name.hashCode ^
      height.hashCode ^
      hasavif.hashCode ^
      hasavifsmalltn.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GalleryFiles &&
          runtimeType == other.runtimeType &&
          width == other.width &&
          hash == other.hash &&
          haswebp == other.haswebp &&
          name == other.name &&
          height == other.height &&
          hasavif == other.hasavif &&
          hasavifsmalltn == other.hasavifsmalltn;
}

class GalleryInfo {
  final String id;
  final String title;
  final String? japaneseTitle;
  final String? language;
  final String? coverUrl;
  final String type;
  final String date;
  final List<Artist>? artists;
  final List<Group>? groups;
  final List<Parody>? parodys;
  final List<Tag>? tags;
  final Int32List related;
  final List<Language> languages;
  final List<Character>? characters;
  final Int32List? sceneIndexes;
  final List<GalleryFiles> files;

  const GalleryInfo({
    required this.id,
    required this.title,
    this.japaneseTitle,
    this.language,
    this.coverUrl,
    required this.type,
    required this.date,
    this.artists,
    this.groups,
    this.parodys,
    this.tags,
    required this.related,
    required this.languages,
    this.characters,
    this.sceneIndexes,
    required this.files,
  });

  static Future<GalleryInfo> newEmpty({dynamic hint}) =>
      RustLib.instance.api.galleryInfoNewEmpty(hint: hint);

  @override
  int get hashCode =>
      id.hashCode ^
      title.hashCode ^
      japaneseTitle.hashCode ^
      language.hashCode ^
      coverUrl.hashCode ^
      type.hashCode ^
      date.hashCode ^
      artists.hashCode ^
      groups.hashCode ^
      parodys.hashCode ^
      tags.hashCode ^
      related.hashCode ^
      languages.hashCode ^
      characters.hashCode ^
      sceneIndexes.hashCode ^
      files.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is GalleryInfo &&
          runtimeType == other.runtimeType &&
          id == other.id &&
          title == other.title &&
          japaneseTitle == other.japaneseTitle &&
          language == other.language &&
          coverUrl == other.coverUrl &&
          type == other.type &&
          date == other.date &&
          artists == other.artists &&
          groups == other.groups &&
          parodys == other.parodys &&
          tags == other.tags &&
          related == other.related &&
          languages == other.languages &&
          characters == other.characters &&
          sceneIndexes == other.sceneIndexes &&
          files == other.files;
}

class Group {
  final String group;
  final String url;

  const Group({
    required this.group,
    required this.url,
  });

  @override
  int get hashCode => group.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Group &&
          runtimeType == other.runtimeType &&
          group == other.group &&
          url == other.url;
}

class Language {
  final String galleryid;
  final String url;
  final String languageLocalname;
  final String name;

  const Language({
    required this.galleryid,
    required this.url,
    required this.languageLocalname,
    required this.name,
  });

  @override
  int get hashCode =>
      galleryid.hashCode ^
      url.hashCode ^
      languageLocalname.hashCode ^
      name.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Language &&
          runtimeType == other.runtimeType &&
          galleryid == other.galleryid &&
          url == other.url &&
          languageLocalname == other.languageLocalname &&
          name == other.name;
}

class Parody {
  final String parody;
  final String url;

  const Parody({
    required this.parody,
    required this.url,
  });

  @override
  int get hashCode => parody.hashCode ^ url.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Parody &&
          runtimeType == other.runtimeType &&
          parody == other.parody &&
          url == other.url;
}

class Tag {
  final String tag;
  final String url;
  final String? female;
  final String? male;

  const Tag({
    required this.tag,
    required this.url,
    this.female,
    this.male,
  });

  @override
  int get hashCode =>
      tag.hashCode ^ url.hashCode ^ female.hashCode ^ male.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Tag &&
          runtimeType == other.runtimeType &&
          tag == other.tag &&
          url == other.url &&
          female == other.female &&
          male == other.male;
}
