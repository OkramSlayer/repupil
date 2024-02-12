import 'dart:developer';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/gallery.dart';
import 'package:repupil/src/rust/api/models/models.dart';
import 'package:repupil/widgets/Tag/TagList.dart';

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

    getGalleryDataFromId(galleryId: widget.id).then((data) {
      if (mounted) {
        setState(() {
          galleryInfo = data;
        });
        if (data.id.isEmpty) {
          log("Disposed widget with id ${widget.id}");
          WidgetsBinding.instance.addPostFrameCallback((_) {
            dispose();
          });
        }
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    final screenHeight = MediaQuery.of(context).size.height;
    return Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: Card(
          elevation: 4,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
          ),
          child: Container(
            padding: const EdgeInsets.fromLTRB(0, 4, 0, 6),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                ClipRRect(
                  borderRadius: BorderRadius.circular(8),
                  child: Image.network(
                    "https://proassetspdlcom.cdnstatics2.com/usuaris/libros/thumbs/3895f1ab-021f-49cf-b6af-96304998fff5/d_295_510/portada_naruto-n-0172_masashi-kishimoto_202211031033.webp",
                    height: screenHeight * 0.25,
                    fit: BoxFit.cover,
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Id: ${widget.id}',
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          fontSize: 18,
                          color: Colors.black87,
                        ),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        'Title: ${galleryInfo.title}',
                        style: const TextStyle(
                          fontWeight: FontWeight.bold,
                          fontSize: 18,
                          color: Colors.black87,
                        ),
                      ),
                      if (galleryInfo.japaneseTitle != null)
                        Text('Japanese Title: ${galleryInfo.japaneseTitle}',
                            style: TextStyle(
                                fontSize: 16, color: Colors.grey[800])),
                      if (galleryInfo.language != null)
                        Text('Language: ${galleryInfo.language}',
                            style: TextStyle(
                                fontSize: 16, color: Colors.grey[800])),
                      Text('Type: ${galleryInfo.type}',
                          style:
                              TextStyle(fontSize: 16, color: Colors.grey[800])),
                      Text('Date: ${galleryInfo.date}',
                          style:
                              TextStyle(fontSize: 16, color: Colors.grey[800])),
                      if (galleryInfo.tags != null)
                        TagList(tags: galleryInfo.tags!),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ));
  }
}
