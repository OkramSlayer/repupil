import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:repupil/src/rust/api/gallery.dart';
import 'package:repupil/widgets/Gallery/GalleryCard.dart';

class CardList extends StatefulWidget {
  const CardList({super.key});

  @override
  State<CardList> createState() => _CardListState();
}

class _CardListState extends State<CardList> {
  late ScrollController _scrollController;
  Int32List allGalleryIds = Int32List.fromList(List.empty());
  List<int> displayedGalleryIds = [];
  
  int currentPage = 1;
  int pageSize = 25; // Change page size to 25

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _scrollController.addListener(_scrollListener);

    getGalleryIdsFromNozomi(language: "all", area: "", tag: "popular")
        .then((ids) {
      allGalleryIds = ids;
      loadCurrentPage();
    });
  }

  void _scrollListener() {
    if (_scrollController.offset >=
            _scrollController.position.maxScrollExtent &&
        !_scrollController.position.outOfRange) {
      loadNextPage();
    }
  }

  void loadCurrentPage() {
    int startIndex = (currentPage - 1) * pageSize;
    int endIndex = startIndex + pageSize;
    setState(() {
      displayedGalleryIds = allGalleryIds.sublist(startIndex, endIndex);
    });
  }

  void loadNextPage() {
    if ((currentPage * pageSize) < allGalleryIds.length) {
      currentPage++;
      loadCurrentPage();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Text('Page: $currentPage'),
          ),
          Expanded(
            child: ListView.builder(
              controller: _scrollController,
              itemCount: displayedGalleryIds.length + 1,
              itemBuilder: (context, index) {
                if (index < displayedGalleryIds.length) {
                  return GalleryCard(id: displayedGalleryIds[index]);
                } else {
                  return const Center(
                    child: CircularProgressIndicator(),
                  );
                }
              },
            ),
          ),
        ],
      ),
    );
  }
}