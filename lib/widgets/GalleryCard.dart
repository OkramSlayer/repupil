import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/gallery.dart';
import 'package:repupil/src/rust/api/utils/get_data.dart';

class GalleryCard extends StatefulWidget {
  final int id;

  const GalleryCard({super.key, required this.id});

  @override
  State<GalleryCard> createState() => _GalleryCardState();
}

class _GalleryCardState extends State<GalleryCard> {
  late GalleryInfo galleryInfo;

  @override
  void initState() {
    super.initState();
    galleryInfo = GalleryInfo(
      id: '',
      title: '',
      type: '',
      date: '',
      related: Int32List(0),
      languages: [],
      files: [],
    );

    getGalleryDataFromId(galleryId: widget.id)
        .then((data) => setState(() => galleryInfo = data));
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Id: ${widget.id}',
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          Text(
            'Title: ${galleryInfo.title}',
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
          if (galleryInfo.japaneseTitle != null)
            Text('Japanese Title: ${galleryInfo.japaneseTitle}'),
          if (galleryInfo.language != null)
            Text('Language: ${galleryInfo.language}'),
          Text('Type: ${galleryInfo.type}'),
          Text('Date: ${galleryInfo.date}'),
          // Puedes agregar más información aquí según necesites
        ],
      ),
    );
  }
}
