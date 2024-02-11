import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/models/models.dart';
import 'package:repupil/widgets/Tag/Tag.dart';

class TagList extends StatelessWidget {
  final List<Tag> tags;
  const TagList({super.key, required this.tags});

  @override
  Widget build(BuildContext context) {
    return Wrap(
      spacing: 4,
      runSpacing: 4,
      children: tags.map((tag) {
        return TagWidget(tag: tag);
      }).toList(),
    );
  }
}
