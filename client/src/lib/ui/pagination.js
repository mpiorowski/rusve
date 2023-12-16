/** Generate pagination schema
 * @param {number} total
 * @param {number} currentPage
 * @param {number} pageSize
 * @returns {number[]}
 */
function generatePaginationSchema(total, currentPage, pageSize) {
    const totalPages = Math.ceil(total / pageSize);
    const paginationArray = [];

    if (totalPages <= 7) {
        for (let i = 1; i <= totalPages; i++) {
            paginationArray.push(i);
        }
    } else {
        paginationArray.push(1);

        if (currentPage < 4) {
            for (let i = 2; i <= 4; i++) {
                paginationArray.push(i);
            }
            paginationArray.push(0);
        }

        if (currentPage >= 4 && currentPage <= totalPages - 3) {
            paginationArray.push(0);
            for (let i = currentPage - 1; i <= currentPage + 1; i++) {
                paginationArray.push(i);
            }
            paginationArray.push(0);
        }

        if (currentPage > totalPages - 3) {
            paginationArray.push(0);
            for (let i = totalPages - 3; i <= totalPages - 1; i++) {
                paginationArray.push(i);
            }
        }

        paginationArray.push(totalPages);
    }

    return paginationArray;
}

/**
 * Generate pagination data
 * @param {number} total
 * @param {number} currentPage
 * @param {number} pageSize
 * @returns {{
 * start: number,
 * end: number,
 * prev: number,
 * next: number,
 * total: number,
 * schema: number[]
 * }}
 */
export function pagination(total, currentPage, pageSize) {
    const start = (currentPage - 1) * pageSize + 1;
    const end = currentPage * pageSize > total ? total : currentPage * pageSize;
    const prev = currentPage > 1 ? currentPage - 1 : 1;
    const next =
        currentPage < Math.ceil(total / pageSize)
            ? currentPage + 1
            : Math.ceil(total / pageSize);
    const schema = generatePaginationSchema(total, currentPage, pageSize);

    return {
        start,
        end,
        prev,
        next,
        total,
        schema,
    };
}
