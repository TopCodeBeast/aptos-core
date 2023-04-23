import React, { useEffect, useState } from "react";

import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkFrontmatter from "remark-frontmatter";
import remarkGfm from "remark-gfm";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";

const root = "https://raw.githubusercontent.com/aptos-labs/aptos-core/";

const ExternalMarkdown = ({ page }: ExternalMarkdownProps) => {
  const [content, setContent] = useState(null);
  let isMounted = true;

  useEffect(() => {
    const fetchContent = async () => {
      if (!isMounted) {
        return;
      }
      const params = new URLSearchParams(location.search);
      const path = params.get("page");
      const path_root = path.match(".*/doc")[0];
      const branch = params.get("branch");

      const page = `${root}/${branch}/aptos-move/framework/${path}`;

      const response = await fetch(page);
      const raw_content = await response.text();
      const regex_major = /href="(.*(\/.*\/doc\/))?(.*\.md.*)"/g;
      const regex_local = /href="([a-zA-Z_-]+\.md)/g;
      const regex_minor = /page=([a-zA-Z_-]+\.md)/g;
      const regex_markdown = /\(([a-zA-Z_-]+\.md.*)\)/g;
      let redirected = raw_content.replaceAll(
        regex_major,
        `href="/reference/move-framework?branch=${branch}&page=$2$3"`,
      );
      redirected = redirected.replaceAll(regex_local, `href="/reference/move-framework?branch=${branch}&page=$1`);
      redirected = redirected.replaceAll(regex_minor, `branch=${branch}&page=${path_root}/$1`);
      redirected = redirected.replaceAll(
        regex_markdown,
        `(/reference/move-framework?branch=${branch}&page=${path_root}/$1)`,
      );

      const output = await unified()
        .use(remarkParse)
        .use(remarkGfm)
        .use(remarkRehype, { allowDangerousHtml: true })
        .use(rehypeStringify, { allowDangerousHtml: true })
        .process(redirected);

      setContent(output.value);
      // If there's an anchor update the href, if you don't check this, infinite loops, yay.
      if (window.location.hash) {
        window.location.href = window.location;
      }
    };

    fetchContent().catch((err) => console.log(`Error fetching spec: ${err}`));
    return () => {
      isMounted = false;
    };
  }, []);

  return <div dangerouslySetInnerHTML={{ __html: content }} />;
};

interface ExternalMarkdownProps {
  page: string;
}

export default ExternalMarkdown;
