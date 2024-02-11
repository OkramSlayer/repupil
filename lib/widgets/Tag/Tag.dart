import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/models/models.dart';

class TagWidget extends StatelessWidget {
  final Tag tag;

  const TagWidget({
    Key? key,
    required this.tag,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    print("${tag.female}${tag.male}");
    return Container(
      decoration: BoxDecoration(
        color: Colors.lightBlueAccent,
        borderRadius: BorderRadius.circular(20), // Bordes redondeados tipo pill
      ),
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (tag.female != null)
            if (tag.female! == "1")
              const Icon(Icons.female, color: Colors.white),
          if (tag.male != null)
            if (tag.male! == "1") const Icon(Icons.male, color: Colors.white),
          const SizedBox(width: 4),
          Text(
            tag.tag,
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.normal,
            ),
          ), // Texto del tag
        ],
      ),
    );
  }
}
