from icrawler.builtin import BingImageCrawler

crawler = BingImageCrawler(storage={'root_dir': 'tmp'})
crawler.crawl(keyword='rip current beach', max_num=400)
